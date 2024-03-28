use bevy::{math::U16Vec2, prelude::*};

use crate::settings::GameSettings;

use super::game_map::{GameMap, GameMapSize, LOGIC_TILE_SIZE};

/// The position of something on the map, in logical pixels. Components of this type are
/// automatically applied to the entity's transform.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<U16Vec2> for Position {
    #[inline]
    fn from(value: U16Vec2) -> Self {
        Self::new(value.x as i32, value.y as i32)
    }
}

impl From<&U16Vec2> for Position {
    #[inline]
    fn from(value: &U16Vec2) -> Self {
        Self::new(value.x as i32, value.y as i32)
    }
}

impl From<IVec2> for Position {
    #[inline]
    fn from(value: IVec2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<&IVec2> for Position {
    #[inline]
    fn from(value: &IVec2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<Position> for IVec2 {
    #[inline]
    fn from(value: Position) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<&Position> for IVec2 {
    #[inline]
    fn from(value: &Position) -> Self {
        Self::new(value.x, value.y)
    }
}

impl PartialEq<IVec2> for Position {
    #[inline]
    fn eq(&self, other: &IVec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<Position> for IVec2 {
    #[inline]
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

const LOGIC_TILE_SIZE_FLOAT: f32 = LOGIC_TILE_SIZE as f32;

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
        let mut pos = Vec2::new(pos.x as f32, pos.y as f32) / LOGIC_TILE_SIZE_FLOAT;
        pos.y = map_size.height as f32 - pos.y;
        transform.translation =
            ((pos - half_map_size) * tile_size - half_tile_adjustment).extend(0.0);
    }
}
