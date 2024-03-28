use bevy::prelude::*;

/// The size that all BW game logic assumes tiles are (in pixels).
pub const LOGIC_TILE_SIZE: i32 = 32;

#[derive(Component, Default, Reflect)]
pub struct GameMap;

#[derive(Component, Default, Reflect)]
pub struct GameMapSize {
    /// Width of the map in tiles
    pub width: u32,
    /// Height of the map in tiles
    pub height: u32,
}

impl From<GameMapSize> for Vec2 {
    fn from(size: GameMapSize) -> Self {
        Vec2::new(size.width as f32, size.height as f32)
    }
}

impl From<&GameMapSize> for Vec2 {
    fn from(size: &GameMapSize) -> Self {
        Vec2::new(size.width as f32, size.height as f32)
    }
}

#[derive(Bundle, Default)]
pub struct GameMapBundle {
    pub game_map: GameMap,
    pub size: GameMapSize,
    pub spatial: SpatialBundle,
}
