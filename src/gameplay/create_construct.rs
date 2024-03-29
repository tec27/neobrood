use bevy::{ecs::system::Command, prelude::*};

use crate::{
    constructs::{ConstructBundle, OwnedConstruct},
    gamedata::{Construct, ConstructTypeId, LoadingAnim},
    maps::{
        game_map::{GameMap, GameMapSize, LOGIC_TILE_SIZE},
        position::Position,
    },
    states::InGameOnly,
};

struct CreateAndPlaceConstruct {
    construct_type: ConstructTypeId,
    position: Position,
    owner: Option<u8>,
}

impl Command for CreateAndPlaceConstruct {
    fn apply(self, world: &mut World) {
        let position: IVec2 = self.position.into();
        let map_size = world
            .query_filtered::<&GameMapSize, With<GameMap>>()
            .single(world);
        let max_position =
            IVec2::new(map_size.width as i32, map_size.height as i32 - 1) * LOGIC_TILE_SIZE;
        let map_bounds = IRect::from_corners(IVec2::ZERO, max_position - 1);
        let search_bounds =
            IRect::from_corners(position - IVec2::splat(128), position + IVec2::splat(127))
                .intersect(map_bounds);

        let mut constructs = world
            .query::<(&ConstructTypeId, &Position)>()
            .iter(world)
            .collect::<Vec<_>>();
        constructs.sort_by_key(|(_, p)| p.x);

        let spawn_construct = |world: &mut World, position: IVec2| {
            let mut entity = world.spawn((
                ConstructBundle {
                    construct_type: self.construct_type,
                    position: position.into(),
                    ..default()
                },
                InGameOnly,
            ));
            if let Some(owner) = self.owner {
                entity.insert(OwnedConstruct(owner));
            }
            entity.with_children(|builder| {
                builder.spawn(LoadingAnim::new(
                    self.construct_type.def().flingy().sprite().image_id,
                ));
            });
        };

        let construct_rect = self.construct_type.def().bounds.at_pos(position);
        let is_within_map_bounds =
            map_bounds.contains(construct_rect.min) && map_bounds.contains(construct_rect.max);

        let blocking_construct = find_blocking_construct(&constructs, construct_rect);
        if blocking_construct.is_none() && is_within_map_bounds {
            spawn_construct(world, position);
            return;
        }

        let mut offset = blocking_construct
            .map(|(c, _)| {
                let placed = self.construct_type.def().bounds;
                let blocking = c.def().bounds;
                // Offset the search by the bottom/right of the blocking construct, plus the top/left of
                // the placed construct
                IVec2::new(
                    (placed.left + blocking.right + 1).max(8),
                    (placed.top + blocking.bottom + 1).max(8),
                )
            })
            .unwrap_or(IVec2::new(8, 8));

        loop {
            let next_min = position - offset;
            let next_max = position + offset;

            if next_min.x < search_bounds.min.x
                && next_min.y < search_bounds.min.y
                && next_max.x > search_bounds.max.x
                && next_max.y > search_bounds.max.y
            {
                // TODO(tec27): Exceeded search bounds (e.g. we failed to place the construct,
                // need to notify things in some way (event?))
                break;
            }

            if let Some(found) = search_for_empty_position(
                self.construct_type.def(),
                next_min,
                offset,
                search_bounds,
                map_bounds,
                &constructs,
            ) {
                spawn_construct(world, found);
                return;
            }

            offset += IVec2::splat(16);
        }
    }
}

fn find_blocking_construct<'a>(
    constructs: &'a [(&'a ConstructTypeId, &'a Position)],
    placed_bounds: IRect,
) -> Option<&'a (&'a ConstructTypeId, &'a Position)> {
    // TODO(tec27): This search can be more efficient because the list is sorted by x position
    // but probably also we should use some spatial index for this
    constructs.iter().find(|(&c, &p)| {
        // TODO(tec27): Flying units should only be blocked by other flying units
        // TODO(tec27): Flying production buildings should block the things they produce
        // TODO(tec27): Don't check against dead units
        !c.def()
            .bounds
            .at_pos(p.into())
            .intersect(placed_bounds)
            .is_empty()
    })
}

fn search_for_empty_position(
    placed: &Construct,
    position: IVec2,
    offset: IVec2,
    search_bounds: IRect,
    map_bounds: IRect,
    constructs: &[(&ConstructTypeId, &Position)],
) -> Option<IVec2> {
    // Potential placements are quantized to mini-tiles (8x8 logical pixels)
    // We will try to place the construct along the edges of this rectangle, starting in the
    // bottom-left corner and moving counter-clockwise around all of the edges
    let mut placement_rect = IRect::from_corners(
        position & !7,
        // This value gets rounded up to the nearest minitile (+7 makes this work like `ceil`)
        (position + (offset * 2) + 7) & !7,
    )
    .intersect(search_bounds);

    let placed_size = placed.bounds.size();

    // Start the search a little right of the left edge (if there is space to do so)
    if offset.x > placed_size.x {
        placement_rect.min.x += (placed_size.x + 7) & !7;
    }

    // Search along the bottom edge
    let mut cur_bounds = placed
        .bounds
        .at_pos(IVec2::new(placement_rect.min.x, placement_rect.max.y));
    let mut x = placement_rect.min.x;
    while x <= placement_rect.max.x {
        if cur_bounds.intersect(map_bounds) == cur_bounds {
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                // Shove the left edge of the search bounds to the center of the blocking construct,
                // then add the right size of blocking construct to clear its bounds
                let mut inc = (blocking.1.x - cur_bounds.min.x) + blocking.0.def().bounds.right;
                // Push inc to the next quantized boundary
                inc += (8 - ((x + inc) & 7)) & 7;
                cur_bounds.min.x += inc;
                cur_bounds.max.x += inc;
                x += inc;
                continue;
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(x, placement_rect.max.y);
                return Some(pos);
            }
        }

        cur_bounds.min.x += 8;
        cur_bounds.max.x += 8;
        x += 8;
    }

    // Search along right edge
    let mut cur_bounds = placed
        .bounds
        .at_pos(IVec2::new(placement_rect.max.x, placement_rect.max.y));
    let mut y = placement_rect.max.y;
    while y >= placement_rect.min.y {
        if cur_bounds.intersect(map_bounds) == cur_bounds {
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                // Shove the bottom edge of the search bounds to the center of the blocking
                // construct, then add the top size of blocking construct plus 1 to clear its bounds
                let mut dec = cur_bounds.max.y - (blocking.1.y - blocking.0.def().bounds.top);
                dec += (8 - ((y - dec) & 7)) & 7;
                cur_bounds.min.y -= dec;
                cur_bounds.max.y -= dec;
                y -= dec;
                continue;
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(placement_rect.max.x, y);
                return Some(pos);
            }
        }

        cur_bounds.min.y -= 8;
        cur_bounds.max.y -= 8;
        y -= 8;
    }

    // Adjust the placement rect back to the original position if we adjusted it above
    // TODO(tec27): Deal with this differently, this code is super brittle :)
    if offset.x > placed_size.x {
        placement_rect.min.x -= (placed_size.x + 7) & !7;
    }

    // Search along top edge
    let mut cur_bounds = placed
        .bounds
        .at_pos(IVec2::new(placement_rect.max.x, placement_rect.min.y));
    x = placement_rect.max.x;
    while x >= placement_rect.min.x {
        if cur_bounds.intersect(map_bounds) == cur_bounds {
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                // Shove the right edge of the search bounds to the center of the blocking
                // construct, then add the left size of blocking construct plus 1 to clear its
                // bounds
                let mut dec = cur_bounds.max.x - (blocking.1.x - blocking.0.def().bounds.left);
                dec += (8 - ((x - dec) & 7)) & 7;
                cur_bounds.min.x -= dec;
                cur_bounds.max.x -= dec;
                x -= dec;
                continue;
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(x, placement_rect.min.y);
                return Some(pos);
            }
        }

        cur_bounds.min.x -= 8;
        cur_bounds.max.x -= 8;
        x -= 8;
    }

    // Search along left edge
    let mut cur_bounds = placed
        .bounds
        .at_pos(IVec2::new(placement_rect.min.x, placement_rect.min.y));
    let mut y = placement_rect.min.y;
    while y <= placement_rect.max.y {
        if cur_bounds.intersect(map_bounds) == cur_bounds {
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                // Shove the top edge of the search bounds to the center of the blocking
                // construct, then add the bottom size of blocking construct plus 1 to clear its
                // bounds
                let mut inc = blocking.1.y - cur_bounds.min.y + blocking.0.def().bounds.bottom;
                inc += (8 - ((y + inc) & 7)) & 7;
                cur_bounds.min.y += inc;
                cur_bounds.max.y += inc;
                y += inc;
                continue;
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(placement_rect.min.x, y);
                return Some(pos);
            }
        }

        cur_bounds.min.y += 8;
        cur_bounds.max.y += 8;
        y += 8;
    }

    None
}

pub trait CreateConstructExt {
    fn create_and_place_construct(
        &mut self,
        construct_type: ConstructTypeId,
        position: Position,
        owner: Option<u8>,
    );
}

impl<'w, 's> CreateConstructExt for Commands<'w, 's> {
    /// Creates and places a construct of the given type as close as possible to the given position.
    fn create_and_place_construct(
        &mut self,
        construct_type: ConstructTypeId,
        position: Position,
        owner: Option<u8>,
    ) {
        // TODO(tec27): This can fail in some cases, how do we handle that? Events?
        self.add(CreateAndPlaceConstruct {
            construct_type,
            position,
            owner,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::maps::game_map::GameMapBundle;

    use super::*;

    #[test]
    fn bottleneck_right_side_scv_placement() {
        let mut app = App::new();

        app.world.spawn(GameMapBundle {
            size: GameMapSize {
                width: 128,
                height: 128,
            },
            ..default()
        });
        let hq_position = IVec2::new(3808, 2384);
        app.world.spawn(ConstructBundle {
            construct_type: ConstructTypeId::TerranCommandCenter,
            position: hq_position.into(),
            ..default()
        });
        app.update();

        let expected_positions = [
            (3760, 2440),
            (3784, 2440),
            (3808, 2440),
            (3832, 2440),
            (3856, 2440),
            (3880, 2440),
            (3880, 2410),
            (3880, 2382),
            (3880, 2358),
            (3880, 2334),
            (3850, 2328),
            (3822, 2328),
            (3798, 2328),
            (3774, 2328),
            (3750, 2328),
            (3736, 2352),
            (3736, 2376),
            (3736, 2400),
            (3736, 2424),
            (3726, 2312),
            (3720, 2448),
            (3728, 2472),
            (3752, 2472),
            (3776, 2472),
            (3800, 2472),
            (3824, 2472),
            (3848, 2472),
            (3872, 2472),
            (3896, 2472),
            (3912, 2442),
            (3912, 2414),
            (3912, 2390),
            (3912, 2366),
            (3912, 2342),
            (3912, 2318),
            (3882, 2296),
            (3854, 2296),
            (3830, 2296),
            (3806, 2296),
            (3782, 2296),
            (3758, 2296),
            (3704, 2336),
            (3704, 2360),
            (3704, 2384),
            (3704, 2408),
            (3704, 2472),
            (3920, 2488),
            (3928, 2294),
            (3734, 2280),
            (3710, 2280),
            (3688, 2304),
            (3688, 2432),
            (3704, 2504),
            (3728, 2504),
            (3752, 2504),
            (3776, 2504),
            (3800, 2504),
            (3824, 2504),
            (3848, 2504),
            (3872, 2504),
            (3896, 2504),
            (3935, 2458),
            (3935, 2430),
            (3935, 2406),
            (3935, 2382),
            (3935, 2358),
            (3935, 2334),
            (3935, 2270),
            (3912, 2264),
            (3882, 2264),
            (3854, 2264),
            (3830, 2264),
            (3806, 2264),
            (3782, 2264),
            (3758, 2264),
            (3686, 2264),
            (3680, 2328),
            (3680, 2352),
            (3680, 2376),
            (3680, 2400),
            (3680, 2456),
            (3680, 2480),
            (3680, 2504),
        ]
        .iter()
        .map(|(x, y)| IVec2::new(*x, *y))
        .collect::<Vec<_>>();

        let check_expected_pos =
            |In(expected): In<Option<(IVec2, usize)>>,
             query: Query<(&ConstructTypeId, &Position), Added<ConstructTypeId>>| {
                if let Some((expected, index)) = expected {
                    // NOTE(tec27): Not using single here because the first run will see the
                    // Command Center as well
                    let mut new_units = query
                        .iter()
                        .filter(|(c, _)| **c == ConstructTypeId::TerranScv);
                    let new_unit = new_units.next().unwrap();
                    assert_eq!(
                        new_unit,
                        (&ConstructTypeId::TerranScv, &expected.into()),
                        "index {index} has incorrect position"
                    );

                    assert_eq!(new_units.count(), 0);
                } else {
                    assert!(query.is_empty());
                }
            };
        let mut check_expected_pos_system = IntoSystem::into_system(check_expected_pos);
        check_expected_pos_system.initialize(&mut app.world);

        for i in 0..expected_positions.len() {
            // TODO(tec27): Unsure how to get a Commands but might be nice to use that instead
            let command = CreateAndPlaceConstruct {
                construct_type: ConstructTypeId::TerranScv,
                position: hq_position.into(),
                owner: None,
            };
            command.apply(&mut app.world);
            app.update();

            let expected = expected_positions[i];
            check_expected_pos_system.run(Some((expected, i)), &mut app.world);
        }

        // Check that the next placement would fall outside the search bounds (e.g. building exit
        // is blocked)
        let command = CreateAndPlaceConstruct {
            construct_type: ConstructTypeId::TerranScv,
            position: hq_position.into(),
            owner: None,
        };
        command.apply(&mut app.world);
        app.update();

        check_expected_pos_system.run(None, &mut app.world);
    }

    #[test]
    fn bottleneck_left_side_marines_around_barracks() {
        // This replicates a "hacked" version of the game where the initial Command Center is
        // instead a Barracks and we only spawn marines around it
        let mut app = App::new();

        app.world.spawn(GameMapBundle {
            size: GameMapSize {
                width: 128,
                height: 128,
            },
            ..default()
        });
        let hq_position = IVec2::new(288, 2416);
        app.world.spawn(ConstructBundle {
            construct_type: ConstructTypeId::TerranBarracks,
            position: hq_position.into(),
            ..default()
        });
        app.update();

        let expected_positions = [
            (240, 2464),
            (264, 2464),
            (288, 2464),
            (312, 2464),
            (336, 2464),
            (360, 2464),
            (360, 2440),
            (360, 2416),
            (360, 2392),
            (360, 2368),
            (230, 2368),
            (216, 2392),
            (216, 2416),
            (216, 2440),
            (216, 2464),
            (342, 2352),
            (322, 2352),
            (298, 2352),
            (274, 2352),
            (250, 2352),
            (210, 2352),
            (208, 2496),
            (232, 2496),
            (256, 2496),
            (280, 2496),
            (304, 2496),
            (328, 2496),
            (352, 2496),
            (376, 2496),
            (392, 2472),
            (392, 2448),
            (392, 2424),
            (392, 2400),
            (392, 2376),
            (392, 2352),
            (374, 2336),
            (186, 2336),
            (184, 2360),
            (184, 2384),
            (184, 2408),
            (184, 2432),
            (184, 2456),
            (184, 2480),
            (400, 2512),
            (408, 2328),
            (354, 2320),
            (330, 2320),
            (306, 2320),
            (282, 2320),
            (258, 2320),
            (234, 2320),
            (210, 2320),
            (168, 2320),
            (168, 2504),
            (184, 2528),
            (208, 2528),
            (232, 2528),
            (256, 2528),
            (280, 2528),
            (304, 2528),
            (328, 2528),
            (352, 2528),
            (376, 2528),
            (415, 2488),
            (415, 2464),
            (415, 2440),
            (415, 2416),
            (415, 2392),
            (415, 2368),
            (415, 2304),
            (396, 2304),
            (374, 2304),
            (186, 2304),
            (160, 2344),
            (160, 2368),
            (160, 2392),
            (160, 2416),
            (160, 2440),
            (160, 2464),
            (160, 2528),
            (400, 2543),
            (354, 2288),
            (330, 2288),
            (306, 2288),
            (282, 2288),
            (258, 2288),
            (234, 2288),
            (210, 2288),
            (162, 2288),
        ]
        .iter()
        .map(|(x, y)| IVec2::new(*x, *y))
        .collect::<Vec<_>>();

        let check_expected_pos =
            |In(expected): In<Option<(IVec2, usize)>>,
             query: Query<(&ConstructTypeId, &Position), Added<ConstructTypeId>>| {
                if let Some((expected, index)) = expected {
                    // NOTE(tec27): Not using single here because the first run will see the
                    // building as well
                    let mut new_units = query
                        .iter()
                        .filter(|(c, _)| **c == ConstructTypeId::TerranMarine);
                    let new_unit = new_units.next().unwrap();
                    assert_eq!(
                        new_unit,
                        (&ConstructTypeId::TerranMarine, &expected.into()),
                        "index {index} has incorrect position"
                    );

                    assert_eq!(new_units.count(), 0);
                } else {
                    assert!(query.is_empty());
                }
            };
        let mut check_expected_pos_system = IntoSystem::into_system(check_expected_pos);
        check_expected_pos_system.initialize(&mut app.world);

        for i in 0..expected_positions.len() {
            // TODO(tec27): Unsure how to get a Commands but might be nice to use that instead
            let command = CreateAndPlaceConstruct {
                construct_type: ConstructTypeId::TerranMarine,
                position: hq_position.into(),
                owner: None,
            };
            command.apply(&mut app.world);
            app.update();

            let expected = expected_positions[i];
            check_expected_pos_system.run(Some((expected, i)), &mut app.world);
        }

        // Check that the next placement would fall outside the search bounds (e.g. building exit
        // is blocked)
        let command = CreateAndPlaceConstruct {
            construct_type: ConstructTypeId::TerranMarine,
            position: hq_position.into(),
            owner: None,
        };
        command.apply(&mut app.world);
        app.update();

        check_expected_pos_system.run(None, &mut app.world);
    }

    #[test]
    fn bottleneck_left_side_ghosts_around_barracks() {
        // This replicates a "hacked" version of the game where the initial Command Center is
        // instead a Barracks and we only spawn ghosts around it
        let mut app = App::new();

        app.world.spawn(GameMapBundle {
            size: GameMapSize {
                width: 128,
                height: 128,
            },
            ..default()
        });
        let hq_position = IVec2::new(288, 2416);
        app.world.spawn(ConstructBundle {
            construct_type: ConstructTypeId::TerranBarracks,
            position: hq_position.into(),
            ..default()
        });
        app.update();

        let expected_positions = [
            (232, 2464),
            (248, 2464),
            (264, 2464),
            (280, 2464),
            (296, 2464),
            (312, 2464),
            (328, 2464),
            (344, 2464),
            (360, 2464),
            (360, 2436),
            (360, 2412),
            (360, 2388),
            (232, 2368),
            (216, 2368),
            (216, 2392),
            (216, 2416),
            (216, 2440),
            (216, 2464),
            (376, 2480),
            (376, 2452),
            (376, 2428),
            (376, 2404),
            (376, 2380),
            (376, 2356),
            (354, 2352),
            (334, 2352),
            (318, 2352),
            (302, 2352),
            (286, 2352),
            (270, 2352),
            (254, 2352),
            (200, 2352),
            (200, 2376),
            (200, 2400),
            (200, 2424),
            (200, 2448),
            (200, 2472),
            (200, 2496),
            (216, 2496),
            (232, 2496),
            (248, 2496),
            (264, 2496),
            (280, 2496),
            (296, 2496),
            (312, 2496),
            (328, 2496),
            (344, 2496),
            (360, 2496),
            (392, 2496),
            (392, 2468),
            (392, 2444),
            (392, 2420),
            (392, 2396),
            (392, 2372),
            (392, 2348),
            (238, 2336),
            (222, 2336),
            (184, 2336),
            (184, 2360),
            (184, 2384),
            (184, 2408),
            (184, 2432),
            (184, 2456),
            (184, 2480),
            (184, 2512),
            (376, 2512),
            (408, 2512),
            (408, 2484),
            (408, 2460),
            (408, 2436),
            (408, 2412),
            (408, 2388),
            (408, 2364),
            (408, 2340),
            (386, 2320),
            (366, 2320),
            (350, 2320),
            (334, 2320),
            (318, 2320),
            (302, 2320),
            (286, 2320),
            (270, 2320),
            (254, 2320),
            (206, 2320),
            (168, 2320),
            (168, 2344),
            (168, 2368),
            (168, 2392),
            (168, 2416),
            (168, 2440),
            (168, 2464),
            (168, 2488),
            (168, 2512),
            (200, 2528),
            (216, 2528),
            (232, 2528),
            (248, 2528),
            (264, 2528),
            (280, 2528),
            (296, 2528),
            (312, 2528),
            (328, 2528),
            (344, 2528),
            (360, 2528),
            (392, 2528),
            (415, 2316),
            (238, 2304),
            (222, 2304),
            (190, 2304),
            (176, 2543),
            (376, 2543),
            (408, 2543),
            (415, 2292),
            (400, 2288),
            (378, 2288),
            (358, 2288),
            (342, 2288),
            (326, 2288),
            (310, 2288),
            (294, 2288),
            (278, 2288),
            (262, 2288),
            (206, 2288),
            (174, 2288),
            (160, 2536),
        ]
        .iter()
        .map(|(x, y)| IVec2::new(*x, *y))
        .collect::<Vec<_>>();

        let check_expected_pos =
            |In(expected): In<Option<(IVec2, usize)>>,
             query: Query<(&ConstructTypeId, &Position), Added<ConstructTypeId>>| {
                if let Some((expected, index)) = expected {
                    // NOTE(tec27): Not using single here because the first run will see the
                    // building as well
                    let mut new_units = query
                        .iter()
                        .filter(|(c, _)| **c == ConstructTypeId::TerranGhost);
                    let new_unit = new_units.next().unwrap();
                    assert_eq!(
                        new_unit,
                        (&ConstructTypeId::TerranGhost, &expected.into()),
                        "index {index} has incorrect position"
                    );

                    assert_eq!(new_units.count(), 0);
                } else {
                    assert!(query.is_empty());
                }
            };
        let mut check_expected_pos_system = IntoSystem::into_system(check_expected_pos);
        check_expected_pos_system.initialize(&mut app.world);

        for i in 0..expected_positions.len() {
            // TODO(tec27): Unsure how to get a Commands but might be nice to use that instead
            let command = CreateAndPlaceConstruct {
                construct_type: ConstructTypeId::TerranGhost,
                position: hq_position.into(),
                owner: None,
            };
            command.apply(&mut app.world);
            app.update();

            let expected = expected_positions[i];
            check_expected_pos_system.run(Some((expected, i)), &mut app.world);
        }

        // Check that the next placement would fall outside the search bounds (e.g. building exit
        // is blocked)
        let command = CreateAndPlaceConstruct {
            construct_type: ConstructTypeId::TerranGhost,
            position: hq_position.into(),
            owner: None,
        };
        command.apply(&mut app.world);
        app.update();

        check_expected_pos_system.run(None, &mut app.world);
    }

    #[test]
    fn bottleneck_left_side_tanks_around_factory() {
        // This replicates a "hacked" version of the game where the initial Command Center is
        // instead a Factory and we only spawn tanks around it
        let mut app = App::new();

        app.world.spawn(GameMapBundle {
            size: GameMapSize {
                width: 128,
                height: 128,
            },
            ..default()
        });
        let hq_position = IVec2::new(288, 2416);
        app.world.spawn(ConstructBundle {
            construct_type: ConstructTypeId::TerranFactory,
            position: hq_position.into(),
            ..default()
        });
        app.update();

        let expected_positions = [
            (240, 2480),
            (272, 2480),
            (304, 2480),
            (336, 2480),
            (368, 2480),
            (368, 2448),
            (368, 2416),
            (368, 2384),
            (368, 2352),
            (336, 2352),
            (304, 2352),
            (272, 2352),
            (240, 2352),
            (208, 2352),
            (208, 2384),
            (208, 2416),
            (208, 2448),
            (208, 2480),
            (208, 2512),
            (240, 2512),
            (272, 2512),
            (304, 2512),
            (336, 2512),
            (368, 2512),
            (400, 2512),
            (400, 2480),
            (400, 2448),
            (400, 2416),
            (400, 2384),
            (400, 2352),
            (400, 2320),
            (368, 2320),
            (336, 2320),
            (304, 2320),
            (272, 2320),
            (240, 2320),
            (208, 2320),
            (176, 2320),
            (176, 2352),
            (176, 2384),
            (176, 2416),
            (176, 2448),
            (176, 2480),
            (176, 2512),
            (415, 2288),
            (382, 2288),
            (348, 2288),
            (312, 2288),
            (280, 2288),
            (248, 2288),
            (216, 2288),
            (184, 2288),
        ]
        .iter()
        .map(|(x, y)| IVec2::new(*x, *y))
        .collect::<Vec<_>>();

        let check_expected_pos =
            |In(expected): In<Option<(IVec2, usize)>>,
             query: Query<(&ConstructTypeId, &Position), Added<ConstructTypeId>>| {
                if let Some((expected, index)) = expected {
                    // NOTE(tec27): Not using single here because the first run will see the
                    // building as well
                    let mut new_units = query
                        .iter()
                        .filter(|(c, _)| **c == ConstructTypeId::TerranSiegeTank);
                    let new_unit = new_units.next().unwrap();
                    assert_eq!(
                        new_unit,
                        (&ConstructTypeId::TerranSiegeTank, &expected.into()),
                        "index {index} has incorrect position"
                    );

                    assert_eq!(new_units.count(), 0);
                } else {
                    assert!(query.is_empty());
                }
            };
        let mut check_expected_pos_system = IntoSystem::into_system(check_expected_pos);
        check_expected_pos_system.initialize(&mut app.world);

        for i in 0..expected_positions.len() {
            // TODO(tec27): Unsure how to get a Commands but might be nice to use that instead
            let command = CreateAndPlaceConstruct {
                construct_type: ConstructTypeId::TerranSiegeTank,
                position: hq_position.into(),
                owner: None,
            };
            command.apply(&mut app.world);
            app.update();

            let expected = expected_positions[i];
            check_expected_pos_system.run(Some((expected, i)), &mut app.world);
        }

        // Check that the next placement would fall outside the search bounds (e.g. building exit
        // is blocked)
        let command = CreateAndPlaceConstruct {
            construct_type: ConstructTypeId::TerranSiegeTank,
            position: hq_position.into(),
            owner: None,
        };
        command.apply(&mut app.world);
        app.update();

        check_expected_pos_system.run(None, &mut app.world);
    }

    #[test]
    fn corner_starts_bottom_right_siege_tanks_around_factory() {
        // This replicates a "hacked" version of the game where the initial Command Center is
        // instead a Factory and we only spawn tanks around it
        let mut app = App::new();

        app.world.spawn(GameMapBundle {
            size: GameMapSize {
                width: 64,
                height: 64,
            },
            ..default()
        });
        let hq_position = IVec2::new(1984, 1936);
        app.world.spawn(ConstructBundle {
            construct_type: ConstructTypeId::TerranFactory,
            position: hq_position.into(),
            ..default()
        });
        app.update();

        let expected_positions = [
            (2031, 1872),
            (1998, 1872),
            (1964, 1872),
            (1928, 1872),
            (1904, 1904),
            (1904, 1936),
            (1904, 1968),
            (1896, 1856),
            (2031, 1840),
            (1998, 1840),
            (1964, 1840),
            (1928, 1840),
            (1872, 1888),
            (1872, 1920),
            (1872, 1952),
            (1872, 1984),
            (1896, 1824),
            (1864, 1824),
            (1856, 1856),
            (2031, 1808),
            (1998, 1808),
            (1964, 1808),
            (1928, 1808),
        ]
        .iter()
        .map(|(x, y)| IVec2::new(*x, *y))
        .collect::<Vec<_>>();

        let check_expected_pos =
            |In(expected): In<Option<(IVec2, usize)>>,
             query: Query<(&ConstructTypeId, &Position), Added<ConstructTypeId>>| {
                if let Some((expected, index)) = expected {
                    // NOTE(tec27): Not using single here because the first run will see the
                    // building as well
                    let mut new_units = query
                        .iter()
                        .filter(|(c, _)| **c == ConstructTypeId::TerranSiegeTank);
                    let new_unit = new_units.next();
                    assert!(
                        new_unit.is_some(),
                        "expected to be able to place index {index} but couldn't"
                    );
                    assert_eq!(
                        new_unit.unwrap(),
                        (&ConstructTypeId::TerranSiegeTank, &expected.into()),
                        "index {index} has incorrect position"
                    );

                    assert_eq!(new_units.count(), 0);
                } else {
                    assert!(query.is_empty());
                }
            };
        let mut check_expected_pos_system = IntoSystem::into_system(check_expected_pos);
        check_expected_pos_system.initialize(&mut app.world);

        for i in 0..expected_positions.len() {
            // TODO(tec27): Unsure how to get a Commands but might be nice to use that instead
            let command = CreateAndPlaceConstruct {
                construct_type: ConstructTypeId::TerranSiegeTank,
                position: hq_position.into(),
                owner: None,
            };
            command.apply(&mut app.world);
            app.update();

            let expected = expected_positions[i];
            check_expected_pos_system.run(Some((expected, i)), &mut app.world);
        }

        // Check that the next placement would fall outside the search bounds (e.g. building exit
        // is blocked)
        let command = CreateAndPlaceConstruct {
            construct_type: ConstructTypeId::TerranSiegeTank,
            position: hq_position.into(),
            owner: None,
        };
        command.apply(&mut app.world);
        app.update();

        check_expected_pos_system.run(None, &mut app.world);
    }

    #[test]
    fn corner_starts_top_left_siege_tanks_around_factory() {
        // This replicates a "hacked" version of the game where the initial Command Center is
        // instead a Factory and we only spawn tanks around it
        let mut app = App::new();

        app.world.spawn(GameMapBundle {
            size: GameMapSize {
                width: 64,
                height: 64,
            },
            ..default()
        });
        let hq_position = IVec2::new(64, 48);
        app.world.spawn(ConstructBundle {
            construct_type: ConstructTypeId::TerranFactory,
            position: hq_position.into(),
            ..default()
        });
        app.update();

        let expected_positions = [
            (32, 112),
            (64, 112),
            (96, 112),
            (128, 112),
            (144, 80),
            (144, 48),
            (144, 16),
            (160, 128),
            (32, 144),
            (64, 144),
            (96, 144),
            (128, 144),
            (176, 96),
            (176, 64),
            (176, 32),
            (160, 160),
        ]
        .iter()
        .map(|(x, y)| IVec2::new(*x, *y))
        .collect::<Vec<_>>();

        let check_expected_pos =
            |In(expected): In<Option<(IVec2, usize)>>,
             query: Query<(&ConstructTypeId, &Position), Added<ConstructTypeId>>| {
                if let Some((expected, index)) = expected {
                    // NOTE(tec27): Not using single here because the first run will see the
                    // building as well
                    let mut new_units = query
                        .iter()
                        .filter(|(c, _)| **c == ConstructTypeId::TerranSiegeTank);
                    let new_unit = new_units.next();
                    assert!(
                        new_unit.is_some(),
                        "expected to be able to place index {index} but couldn't"
                    );
                    assert_eq!(
                        new_unit.unwrap(),
                        (&ConstructTypeId::TerranSiegeTank, &expected.into()),
                        "index {index} has incorrect position"
                    );

                    assert_eq!(new_units.count(), 0);
                } else {
                    assert!(query.is_empty());
                }
            };
        let mut check_expected_pos_system = IntoSystem::into_system(check_expected_pos);
        check_expected_pos_system.initialize(&mut app.world);

        for i in 0..expected_positions.len() {
            // TODO(tec27): Unsure how to get a Commands but might be nice to use that instead
            let command = CreateAndPlaceConstruct {
                construct_type: ConstructTypeId::TerranSiegeTank,
                position: hq_position.into(),
                owner: None,
            };
            command.apply(&mut app.world);
            app.update();

            let expected = expected_positions[i];
            check_expected_pos_system.run(Some((expected, i)), &mut app.world);
        }

        // Check that the next placement would fall outside the search bounds (e.g. building exit
        // is blocked)
        let command = CreateAndPlaceConstruct {
            construct_type: ConstructTypeId::TerranSiegeTank,
            position: hq_position.into(),
            owner: None,
        };
        command.apply(&mut app.world);
        app.update();

        check_expected_pos_system.run(None, &mut app.world);
    }
}
