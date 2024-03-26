use bevy::{ecs::system::Command, math::U16Vec2, prelude::*};

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
        warn!(
            "PLACING UNIT: {:?} at {:?}",
            self.construct_type, self.position
        );

        let position = IVec2::new(self.position.x as i32, self.position.y as i32);
        let map_size = world
            .query_filtered::<&GameMapSize, With<GameMap>>()
            .single(world);
        let max_position = U16Vec2::new(map_size.width as u16, map_size.height as u16 - 1)
            * (LOGIC_TILE_SIZE as u16);
        let map_bounds = IRect::from_corners(IVec2::ZERO, max_position.into());
        let search_bounds =
            IRect::from_corners(position - IVec2::splat(128), position + IVec2::splat(127))
                // TODO(tec27): OpenBW subtracts 1 from max_position when clamping, but this seems
                // incorrect to me. Verify that this implementation gives matching results
                .intersect(map_bounds);

        let mut constructs = world
            .query::<(&ConstructTypeId, &Position)>()
            .iter(world)
            .collect::<Vec<_>>();
        constructs.sort_by_key(|(_, p)| p.x);

        let construct_bounds = self.construct_type.def().bounding_box(position);
        let is_within_map_bounds =
            map_bounds.contains(construct_bounds.min) && map_bounds.contains(construct_bounds.max);
        if !is_within_map_bounds {
            warn!("unit placed outside of map bounds");
            // TODO(tec27): Do something with this to offset the initial search location
        }
        let blocking_construct = find_blocking_construct(
            &constructs,
            self.construct_type.def().bounding_box(position),
        );
        if blocking_construct.is_none() && is_within_map_bounds {
            let mut entity = world.spawn((
                ConstructBundle {
                    construct_type: self.construct_type,
                    position: Position::new(position.x as u16, position.y as u16),
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
            return;
        }

        let mut offset = if let Some((c, _)) = blocking_construct {
            let placed_rect = self.construct_type.def().unit_rect;
            let blocking_rect = c.def().unit_rect;
            // Offset the search by the bottom/right of the blocking construct, plus the top/left of
            // the placed construct
            IVec2::new(
                ((placed_rect.min.x + blocking_rect.max.x + 2) as i32).max(8),
                ((placed_rect.min.y + blocking_rect.max.y + 2) as i32).max(8),
            )
        } else {
            IVec2::new(8, 8)
        };

        loop {
            let next_pos = position - offset;
            if !search_bounds.contains(next_pos) || !search_bounds.contains(position + offset) {
                warn!("unit placement exceeded search bounds");
                break;
            }

            if let Some(found) = search_for_empty_position(
                self.construct_type.def(),
                next_pos,
                offset,
                search_bounds,
                &constructs,
            ) {
                let mut entity = world.spawn((
                    ConstructBundle {
                        construct_type: self.construct_type,
                        position: Position::new(found.x as u16, found.y as u16),
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
    constructs.iter().find(|(&c, &p)| {
        // TODO(tec27): Flying units should only be blocked by other flying units
        // TODO(tec27): Flying production buildings should block the things they produce
        // TODO(tec27): Don't check against dead units
        !c.def()
            .bounding_box(p.into())
            .intersect(placed_bounds)
            .is_empty()
    })
}

fn search_for_empty_position(
    placed: &Construct,
    position: IVec2,
    offset: IVec2,
    global_bounds: IRect,
    constructs: &[(&ConstructTypeId, &Position)],
) -> Option<IVec2> {
    // Potential placements are quantized to mini-tiles (8x8 logical pixels)
    // We will try to place the construct along the edges of this rectangle, starting in the
    // bottom-left corner and moving counter-clockwise around all of the edges
    let mut placement_rect = IRect::from_corners(
        position & !7,
        // This value gets rounded up to the nearest minitile (+7 makes this work like `ceil`)
        (position + offset * 2 + 7) & !7,
    )
    .intersect(global_bounds);

    let placed_dimens = IVec2::new(
        (placed.unit_rect.min.x + placed.unit_rect.max.x) as i32,
        (placed.unit_rect.min.y + placed.unit_rect.max.y) as i32,
    );
    warn!("placed_dimens: {placed_dimens:?}");

    // Start the search a little right of the left edge (if there is space to do so)
    if offset.x > placed_dimens.x + 1 {
        placement_rect.min.x += (placed_dimens.x + 1) & !7;
    }
    warn!("placement_rect: {placement_rect:?}");

    // Search along the bottom edge
    let mut cur_bounds =
        placed.bounding_box(IVec2::new(placement_rect.min.x, placement_rect.max.y));
    let mut x = placement_rect.min.x;
    while x <= placement_rect.max.x {
        if cur_bounds.intersect(global_bounds) == cur_bounds {
            warn!("finding unit blocking: {cur_bounds:?}");
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                warn!("found blocking unit in search path: {blocking:?}");
                // Shove the left edge of the search boudns to the center of the blocking construct,
                // then add the right size of place construct plus 1 to clear its bounds
                let mut inc =
                    (blocking.1.x as i32 - cur_bounds.min.x) + placed.unit_rect.max.x as i32 + 1;
                // Push inc to the next quantized boundary
                inc += (8 - ((x + inc) & 7)) & 7;
                cur_bounds.min.x += inc;
                cur_bounds.max.x += inc;
                x += inc;
                continue;
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(x, placement_rect.max.y);
                warn!("empty position found: {pos:?}");
                return Some(pos);
            }
        } else {
            warn!("cur_bounds is outside search bounds: {cur_bounds:?}");
        }

        cur_bounds.min.x += 8;
        cur_bounds.max.x += 8;
        x += 8;
    }

    // Search along right edge
    let mut cur_bounds =
        placed.bounding_box(IVec2::new(placement_rect.max.x, placement_rect.max.y));
    let mut y = placement_rect.max.y;
    while y >= placement_rect.min.y {
        if cur_bounds.intersect(global_bounds) == cur_bounds {
            warn!("finding unit blocking: {cur_bounds:?}");
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                warn!("found blocking unit in search path: {blocking:?}");
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(placement_rect.max.x, y);
                warn!("empty position found: {pos:?}");
                return Some(pos);
            }
        } else {
            warn!("cur_bounds is outside search bounds: {cur_bounds:?}");
        }

        cur_bounds.min.y -= 8;
        cur_bounds.max.y -= 8;
        y -= 8;
    }

    // Search along top edge
    let mut cur_bounds =
        placed.bounding_box(IVec2::new(placement_rect.max.x, placement_rect.min.y));
    x = placement_rect.max.x;
    while x >= placement_rect.min.x {
        if cur_bounds.intersect(global_bounds) == cur_bounds {
            warn!("finding unit blocking: {cur_bounds:?}");
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                warn!("found blocking unit in search path: {blocking:?}");
                // FIXME: skip area of search path that blocking unit occupies
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(x, placement_rect.min.y);
                warn!("empty position found: {pos:?}");
                return Some(pos);
            }
        } else {
            warn!("cur_bounds is outside search bounds: {cur_bounds:?}");
        }

        cur_bounds.min.x -= 8;
        cur_bounds.max.x -= 8;
        x -= 8;
    }

    // Search along left edge
    let mut cur_bounds =
        placed.bounding_box(IVec2::new(placement_rect.min.x, placement_rect.min.y));
    let mut y = placement_rect.min.y;
    while y <= placement_rect.max.y {
        if cur_bounds.intersect(global_bounds) == cur_bounds {
            warn!("finding unit blocking: {cur_bounds:?}");
            if let Some(blocking) = find_blocking_construct(constructs, cur_bounds) {
                warn!("found blocking unit in search path: {blocking:?}");
                // FIXME: skip area of search path that blocking unit occupies
            } else {
                // TODO(tec27): Check that terrain can fit the unit
                let pos = IVec2::new(placement_rect.min.x, y);
                warn!("empty position found: {pos:?}");
                return Some(pos);
            }
        } else {
            warn!("cur_bounds is outside search bounds: {cur_bounds:?}");
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
