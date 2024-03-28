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
    fn apply(self, mut world: &mut World) {
        let position: IVec2 = self.position.into();
        let map_size = world
            .query_filtered::<&GameMapSize, With<GameMap>>()
            .single(world);
        let max_position =
            IVec2::new(map_size.width as i32, map_size.height as i32 - 1) * LOGIC_TILE_SIZE;
        let map_bounds = IRect::from_corners(IVec2::ZERO, max_position);
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

        let spawn_construct = |world: &mut World, position: IVec2| {
            warn!("Placing {:?} at {position:?}", self.construct_type);
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
        if !is_within_map_bounds {
            warn!("unit placed outside of map bounds");
            // TODO(tec27): Do something with this to offset the initial search location
        }
        let blocking_construct = find_blocking_construct(&constructs, construct_rect);
        if blocking_construct.is_none() && is_within_map_bounds {
            spawn_construct(&mut world, position);
            return;
        }

        let mut offset = blocking_construct
            .map(|(c, _)| {
                let placed = self.construct_type.def().bounds;
                let blocking = c.def().bounds;
                // Offset the search by the bottom/right of the blocking construct, plus the top/left of
                // the placed construct
                IVec2::new(
                    (placed.left + blocking.right + 2).max(8),
                    (placed.top + blocking.bottom + 2).max(8),
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
                spawn_construct(&mut world, found);
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
    if offset.x > placed_size.x + 1 {
        placement_rect.min.x += (placed_size.x + 1 + 7) & !7;
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
                // then add the right size of blocking construct plus 1 to clear its bounds
                let mut inc = blocking.1.x - cur_bounds.min.x + blocking.0.def().bounds.right + 1;
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
                let mut dec = cur_bounds.max.y - (blocking.1.y - blocking.0.def().bounds.top - 1);
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
    if offset.x > placed_size.x + 1 {
        placement_rect.min.x -= (placed_size.x + 1 + 7) & !7;
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
                let mut dec = cur_bounds.max.x - (blocking.1.x - blocking.0.def().bounds.left - 1);
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
                let mut inc = blocking.1.y - cur_bounds.min.y + blocking.0.def().bounds.bottom + 1;
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
