use bevy::prelude::*;

#[derive(Component, Default)]
pub struct GameMap;

#[derive(Bundle, Default)]
pub struct GameMapBundle {
    game_map: GameMap,
    spatial: SpatialBundle,
}
