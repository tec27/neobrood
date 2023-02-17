use anyhow::anyhow;
use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;
use broodmap::chk::terrain::TerrainTileIds;
use broodmap::chk::tileset::Tileset;

use crate::maps::tileset::{
    load_mega_tile_lookup, load_tile_textures, MegaTileFlags, MegaTileInfo,
};

mod tileset;

pub struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_asset::<MapAsset>()
            .init_asset_loader::<MapAssetLoader>()
            .init_resource::<CurrentMap>()
            .add_system(tilemap_init);
    }
}

/// A bevy [AssetLoader] for SCM and SCX files.
#[derive(Default, Debug)]
pub struct MapAssetLoader;

#[derive(Debug, TypeUuid)]
#[uuid = "78325f88-6895-4e38-acc9-1aa90879c261"]
pub struct MapAsset {
    /// Width of the map in tiles.
    pub width: u32,
    /// Height of the map in tiles.
    pub height: u32,
    /// The map's tileset.
    pub tileset: Tileset,
    /// The map's terrain, as a 2D vector of tile IDs (can be converted to mega-tiles via
    /// `mega_tile_lookup`).
    pub terrain: TerrainTileIds,
    /// A hashmap of tile IDs to their mega-tile info.
    pub mega_tile_lookup: HashMap<u16, MegaTileInfo>,
    /// A Vec of handles to textures for each mega-tile.
    pub tile_textures: Vec<Handle<Image>>,
    /// A map of mega-tile IDs -> an index into `tile_textures`.
    pub tile_texture_indices: HashMap<u16, TileTextureIndex>,
}

impl AssetLoader for MapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            // TODO(tec27): At some point we'll need the MPQ to be able to laod other assets
            // (for UMS), but I don't want to deal with the lifetimes for now, so we just drop it
            let (chk, _mpq) = broodmap::extract_chk_from_map(bytes, None, None)?;
            let tileset = chk.tileset();
            let Ok(terrain) = chk.terrain() else {
                return Err(anyhow!("Could not load map's terrain"));
            };

            let mega_tile_lookup = load_mega_tile_lookup(tileset, terrain, load_context).await?;
            info!("Mega tile lookup has {} entries", mega_tile_lookup.len());

            let (tile_textures, tile_texture_indices) =
                load_tile_textures(tileset, &mega_tile_lookup, load_context).await?;
            info!("Loaded {} tile textures", tile_textures.len());

            let map = MapAsset {
                width: chk.width() as u32,
                height: chk.height() as u32,
                tileset,
                terrain: terrain.clone(),
                mega_tile_lookup,
                tile_textures,
                tile_texture_indices,
            };
            load_context.set_default_asset(LoadedAsset::new(map));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["scx", "scm"]
    }
}

#[derive(Resource, Default)]
pub struct CurrentMap {
    pub handle: Handle<MapAsset>,
}

fn tilemap_init(
    mut commands: Commands,
    mut asset_events: EventReader<AssetEvent<MapAsset>>,
    current_map: Res<CurrentMap>,
    map_assets: Res<Assets<MapAsset>>,
) {
    for event in asset_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if *handle == current_map.handle {
                info!("Map loaded!");
                let map = map_assets.get(handle).unwrap();
                create_tilemap(&mut commands, map);
            }
        }
    }
}

fn create_tilemap(commands: &mut Commands, map: &MapAsset) {
    let map_size = TilemapSize {
        x: map.width,
        y: map.height,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            // TODO(tec27): Write a type that handles the creep flag masking automatically when
            // indexing our map
            let tile_id = map.terrain[y as usize][x as usize].id();
            let mega_tile = map.mega_tile_lookup.get(&tile_id).unwrap();
            // Bevy coords start from the bottom-left, rather than top-left like the map data
            let mapped_y = map_size.y - 1 - y;
            let tile_pos = TilePos { x, y: mapped_y };

            // TODO(tec27): Use actual graphics, this is just to see that the tilemap is working
            let walkable_multiplier = if mega_tile.flags.contains(MegaTileFlags::WALKABLE) {
                1.5f32
            } else if mega_tile.flags.contains(MegaTileFlags::PARTIALLY_WALKABLE) {
                1.25f32
            } else {
                1.0f32
            };
            let color = if mega_tile.flags.contains(MegaTileFlags::BLOCKS_VISION) {
                Color::rgb(0.8, 0.8, (0.8 * walkable_multiplier).min(1.0))
            } else if mega_tile.flags.contains(MegaTileFlags::LEVEL_HIGH) {
                Color::rgb(0.65, 0.65, (0.65 * walkable_multiplier).min(1.0))
            } else if mega_tile.flags.contains(MegaTileFlags::LEVEL_MID) {
                Color::rgb(0.5, 0.5, (0.5 * walkable_multiplier).min(1.0))
            } else {
                Color::rgb(0.35, 0.35, (0.35 * walkable_multiplier).min(1.0))
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    color: color.into(),
                    texture_index: *map.tile_texture_indices.get(&mega_tile.id).unwrap(),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // TODO(tec27): Handle different tile sizes depending on resolution:
    // 4k => 128, 2k => 64, SD => 32
    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    let texture_vec = TilemapTexture::Vector(map.tile_textures.clone());

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        texture: texture_vec,
        ..default()
    });
}

// TODO(tec27): Is 0,0 the best spot for the center for us? maybe placing the bottom-left corner
// there would be better?
/// Calculates a [`Transform`] for a tilemap that places it so that its center is at
/// `(0.0, 0.0, 0.0)` in world space.
pub fn get_tilemap_center_transform(
    size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    z: f32,
) -> Transform {
    let low = TilePos::new(0, 0).center_in_world(grid_size, map_type);
    let high = TilePos::new(size.x - 1, size.y - 1).center_in_world(grid_size, map_type);

    let diff = high - low;

    Transform::from_xyz(-diff.x / 2., -diff.y / 2., z)
}
