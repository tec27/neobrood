use bevy::{math::U16Vec2, prelude::*};

use crate::settings::GameSettings;

use super::game_map::{GameMap, GameMapSize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl From<Position> for U16Vec2 {
    fn from(value: Position) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<&Position> for U16Vec2 {
    fn from(value: &Position) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<Position> for Vec2 {
    fn from(value: Position) -> Self {
        Self::new(value.x as f32, value.y as f32)
    }
}

impl From<&Position> for Vec2 {
    fn from(value: &Position) -> Self {
        Self::new(value.x as f32, value.y as f32)
    }
}

/// The size that all BW game logic assumes tiles are (in pixels).
const LOGIC_TILE_SIZE: f32 = 32.0;

/// Updates entities' transforms based on their [Position] component and the current map
/// size/tile size.
pub fn position_to_transform(
    mut positioned: Query<(&Position, &mut Transform), Changed<Position>>,
    map: Query<&GameMapSize, With<GameMap>>,
    settings: Res<GameSettings>,
) {
    let Ok(map_size) = map.get_single() else {
        // No map so there's no positions to convert
        return;
    };
    if positioned.is_empty() {
        // No positions to convert
        return;
    }

    // NOTE(tec27): Currently this does not update positions if the GameMapSize changes, as it's
    // assumed to only change outside of gameplay. If that is not the case this system probably
    // needs to be reworked a bit.

    let half_map_size = Vec2::from(map_size) / 2.0;
    let tile_size = settings.asset_quality.tile_size();
    let half_tile_adjustment = Vec2::new(tile_size.x / 2.0, tile_size.y / 2.0);

    for (pos, mut transform) in positioned.iter_mut() {
        let mut pos = Vec2::from(pos) / LOGIC_TILE_SIZE;
        pos.y = map_size.height as f32 - pos.y;
        transform.translation =
            ((pos - half_map_size) * tile_size - half_tile_adjustment).extend(0.0);
    }
}
