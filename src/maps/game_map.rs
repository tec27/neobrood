use bevy::{prelude::*, utils::HashMap};
use broodmap::chk::terrain::{TerrainTileIds, TileId};
use thiserror::Error;

use super::{
    position::Position,
    tileset::{MegaTileFlags, MegaTileInfo, MiniTileFlags},
};

/// The size that all BW game logic assumes tiles are (in pixels).
pub const LOGIC_TILE_SIZE: i32 = 32;

/// The size that all BW game logic assumes mini-tiles are (in pixels).
pub const LOGIC_MINI_TILE_SIZE: i32 = 8;

#[derive(Component, Default, Reflect)]
pub struct GameMap;

#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
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
    pub terrain: GameMapTerrain,
}

#[derive(Component, Default)]
pub struct GameMapTerrain {
    /// The width of the map in MegaTiles.
    width: usize,
    /// The map's megatiles, organized row-wise from top-left to bottom-right.
    tiles: Vec<MegaTileInfo>,
}

#[derive(Error, Debug)]
pub enum GameMapTerrainError {
    #[error("unknown mega-tile encountered: {0:?}")]
    UnknownMegaTile(TileId),
}

impl GameMapTerrain {
    /// Create a new [GameMapTerrain] from a list of [MegaTileInfo] and the width of the map in
    /// MegaTiles.
    pub fn new(tiles: Vec<MegaTileInfo>, width: usize) -> Self {
        Self { tiles, width }
    }

    /// Creates a new [GameMapTerrain] from a [TerrainTileIds] and a lookup table of mega-tile
    /// information.
    pub fn from_terrain_and_lookup(
        terrain: &TerrainTileIds,
        lookup: &HashMap<u16, MegaTileInfo>,
    ) -> Result<Self, GameMapTerrainError> {
        let tiles = terrain
            .tiles
            .iter()
            .map(|tile_id| {
                lookup
                    .get(&tile_id.id())
                    .cloned()
                    .ok_or(GameMapTerrainError::UnknownMegaTile(*tile_id))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(tiles, terrain.width))
    }

    /// Returns the mega-tile at the given position.
    pub fn tile_at(&self, position: Position) -> Option<&MegaTileInfo> {
        let x = position.x / LOGIC_TILE_SIZE;
        let y = position.y / LOGIC_TILE_SIZE;

        // NOTE(tec27): y check is unnecessary since the get call below will already handle that
        if x < 0 || y < 0 || (x as usize) >= self.width {
            return None;
        }

        self.tiles.get((y as usize) * self.width + (x as usize))
    }

    /// Returns the mini-tile at the given position.
    pub fn mini_tile_at(&self, position: Position) -> Option<MiniTileFlags> {
        self.tile_at(position).map(|tile| {
            let x = (position.x % LOGIC_TILE_SIZE) as usize;
            let y = (position.y % LOGIC_TILE_SIZE) as usize;
            tile.mini_tile_at(x, y)
        })
    }

    /// Returns whether the given position is walkable.
    pub fn is_walkable(&self, position: Position) -> bool {
        let Some(mega) = self.tile_at(position) else {
            return false;
        };

        if mega.flags.contains(MegaTileFlags::WALKABLE)
            || mega.flags.contains(MegaTileFlags::HAS_CREEP)
        {
            return true;
        }
        if mega.flags.contains(MegaTileFlags::PARTIALLY_WALKABLE) {
            let x = (position.x % LOGIC_TILE_SIZE) as usize;
            let y = (position.y % LOGIC_TILE_SIZE) as usize;
            return mega.mini_tile_at(x, y).contains(MiniTileFlags::WALKABLE);
        }

        false
    }
}
