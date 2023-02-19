use anyhow::anyhow;
use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;
use broodmap::chk::terrain::TerrainTileIds;
use broodmap::chk::tileset::Tileset;

use crate::maps::tileset::{load_mega_tile_lookup, load_tile_textures, MegaTileInfo};

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
            // TODO(tec27): At some point we'll need the MPQ to be able to load other assets
            // (for UMS), but I don't want to deal with the lifetimes for now, so we just drop it
            let (chk, _mpq) = broodmap::extract_chk_from_map(bytes, None, None)?;
            let tileset = chk.tileset();
            let Ok(terrain) = chk.terrain() else {
                return Err(anyhow!("Could not load map's terrain"));
            };

            info!("Loading mega tile lookup...");
            let mega_tile_lookup = load_mega_tile_lookup(tileset, terrain, load_context).await?;
            info!("Mega tile lookup has {} entries", mega_tile_lookup.len());

            info!("Loading tileset textures...");
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
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    for event in asset_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if *handle == current_map.handle {
                info!("Map loaded!");
                let map = map_assets.get(handle).unwrap();
                create_tilemap(&mut commands, map, &array_texture_loader);
            }
        }
    }
}

/// How many tiles should be managed as one chunk by the tilemap. Ideally this should be something
/// that all map sizes evenly divide by, or the tilemap will not center properly at (0,0).
/// This value should ensure that even if every tile in the chunk has a unique texture, it will
/// not exceed 2048 textures for the chunk (so keep it at 32 or less!).
const CHUNK_SIZE_TILES: UVec2 = UVec2::splat(32);

fn create_tilemap(
    commands: &mut Commands,
    map: &MapAsset,
    array_texture_loader: &Res<ArrayTextureLoader>,
) {
    let num_chunks = UVec2 {
        x: map.width - 1,
        y: map.height - 1,
    } / CHUNK_SIZE_TILES
        + UVec2::splat(1);

    let mut tilemaps = (0..(num_chunks.x * num_chunks.y))
        .map(|_| {
            (
                commands.spawn_empty().id(),
                TileStorage::empty(CHUNK_SIZE_TILES.into()),
                HashMap::new(),
                Vec::new(),
            )
        })
        .collect::<Vec<_>>();

    for x in 0..map.width {
        for y in 0..map.height {
            let tile_id = map.terrain[y as usize][x as usize].id();
            let mega_tile = map.mega_tile_lookup.get(&tile_id).unwrap();
            let texture = map.tile_textures
                [map.tile_texture_indices.get(&mega_tile.id).unwrap().0 as usize]
                .clone();

            // Bevy coords start from the bottom-left, rather than top-left like the map data
            let y = map.height - 1 - y;
            let chunk = UVec2 { x, y } / CHUNK_SIZE_TILES;
            let (tilemap_entity, tile_storage, texture_ids, textures) =
                &mut tilemaps[(chunk.x + chunk.y * num_chunks.x) as usize];

            let x = x % CHUNK_SIZE_TILES.x;
            let y = y % CHUNK_SIZE_TILES.y;

            let tile_pos = TilePos { x, y };
            let texture_index = *texture_ids.entry(texture.id()).or_insert_with(|| {
                textures.push(texture);
                TileTextureIndex((textures.len() - 1) as u32)
            });

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(*tilemap_entity),
                    texture_index,
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // TODO(tec27): Handle different tile sizes depending on resolution:
    // 4k => 128, 2k => 64, SD => 32
    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let map_type = TilemapType::Square;

    let chunk_max_pos = Vec2::new(
        (num_chunks.x * CHUNK_SIZE_TILES.x) as f32 * tile_size.x / 2.0,
        (num_chunks.y * CHUNK_SIZE_TILES.y) as f32 * tile_size.y / 2.0,
    );

    for (i, (tilemap_entity, tile_storage, _, textures)) in tilemaps.into_iter().enumerate() {
        let chunk_y = i as u32 / num_chunks.x;
        let chunk_x = i as u32 % num_chunks.x;
        let transform = Transform::from_translation(Vec3::new(
            (chunk_x * CHUNK_SIZE_TILES.x) as f32 * tile_size.x - chunk_max_pos.x,
            (chunk_y * CHUNK_SIZE_TILES.y) as f32 * tile_size.y - chunk_max_pos.y,
            0.0,
        ));

        let texture_vec = TilemapTexture::Vector(textures);

        array_texture_loader.add(TilemapArrayTexture {
            texture: texture_vec.clone(),
            tile_size,
            format: TextureFormat::Bc1RgbaUnormSrgb,
            ..default()
        });

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            map_type,
            size: CHUNK_SIZE_TILES.into(),
            storage: tile_storage,
            tile_size,
            transform,
            texture: texture_vec,
            ..default()
        });
    }
}
