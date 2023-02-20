use anyhow::anyhow;
use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::TextureFormat;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::CompressedImageFormats;
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
#[derive(Debug)]
pub struct MapAssetLoader {
    supported_compressed_formats: CompressedImageFormats,
}

impl FromWorld for MapAssetLoader {
    fn from_world(world: &mut World) -> Self {
        let supported_compressed_formats = match world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),
            None => CompressedImageFormats::all(),
        };

        Self {
            supported_compressed_formats,
        }
    }
}

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
    pub tile_texture_indices: HashMap<u16, usize>,
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
            let (tile_textures, tile_texture_indices) = load_tile_textures(
                tileset,
                &mega_tile_lookup,
                load_context,
                self.supported_compressed_formats,
            )
            .await?;
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

/// The maximum amount of unique textures a single tilemap is allowed to reference. This is to avoid
/// panics caused by exceeding the max size of an array texture (generally 2048). When a tilemap
/// would exceed this number, we instead construct a new one with any remaining tiles that need
/// as-of-yet-unreferenced textures.
const MAX_TEXTURES_PER_TILEMAP: usize = 1920;

#[derive(Debug)]
struct TilemapBuilder {
    entity: Entity,
    tile_storage: TileStorage,
    textures: Vec<Handle<Image>>,
}

impl TilemapBuilder {
    pub fn new(commands: &mut Commands, tilemap_size: TilemapSize) -> Self {
        Self {
            entity: commands.spawn_empty().id(),
            tile_storage: TileStorage::empty(tilemap_size),
            textures: Vec::new(),
        }
    }

    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }

    /// Adds a new texture to the Tilemap, returning the index of that texture to be used for the
    /// TileBundle.
    pub fn add_texture(&mut self, texture: Handle<Image>) -> TileTextureIndex {
        let index = self.texture_count();
        self.textures.push(texture);
        TileTextureIndex(index as u32)
    }
}

fn create_tilemap(
    commands: &mut Commands,
    map: &MapAsset,
    array_texture_loader: &Res<ArrayTextureLoader>,
) {
    let tilemap_size = TilemapSize {
        x: map.width,
        y: map.height,
    };

    let mut tilemaps = vec![TilemapBuilder::new(commands, tilemap_size)];
    let mut texture_to_tilemap: HashMap<usize, (usize, TileTextureIndex)> = HashMap::new();

    for x in 0..map.width {
        for y in 0..map.height {
            let tile_id = map.terrain[y as usize][x as usize].id();
            let mega_tile = map.mega_tile_lookup.get(&tile_id).unwrap();
            let texture_index = *map.tile_texture_indices.get(&mega_tile.id).unwrap();
            let (tilemap_index, tilemap_texture_index) =
                match texture_to_tilemap.get(&texture_index) {
                    Some(&(tilemap_index, tilemap_texture_index)) => {
                        (tilemap_index, tilemap_texture_index)
                    }
                    None => {
                        if tilemaps.last().unwrap().texture_count() >= MAX_TEXTURES_PER_TILEMAP {
                            let tilemap = TilemapBuilder::new(commands, tilemap_size);
                            tilemaps.push(tilemap);
                        }

                        let tilemap_index = tilemaps.len() - 1;
                        let last_tilemap = tilemaps.last_mut().unwrap();
                        let tilemap_texture_index =
                            last_tilemap.add_texture(map.tile_textures[texture_index].clone());
                        texture_to_tilemap
                            .insert(texture_index, (tilemap_index, tilemap_texture_index));

                        (tilemap_index, tilemap_texture_index)
                    }
                };

            // Bevy coords start from the bottom-left, rather than top-left like the map data
            let y = map.height - 1 - y;
            let tile_pos = TilePos { x, y };

            let tilemap = &mut tilemaps[tilemap_index];
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap.entity),
                    texture_index: tilemap_texture_index,
                    ..default()
                })
                .id();
            tilemap.tile_storage.set(&tile_pos, tile_entity);
            commands.entity(tilemap.entity).add_child(tile_entity);
        }
    }

    // TODO(tec27): Handle different tile sizes depending on resolution:
    // 4k => 128, 2k => 64, SD => 32
    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let map_type = TilemapType::Square;
    // Center the map at (0,0)
    let transform = Transform::from_translation(Vec3::new(
        -(tilemap_size.x as f32 * tile_size.x / 2.0),
        -(tilemap_size.y as f32 * tile_size.y / 2.0),
        0.0,
    ));

    info!("Creating {} tilemaps", tilemaps.len());
    for tilemap in tilemaps.drain(..) {
        info!("Tilemap has {} textures", tilemap.textures.len());
        let texture_vec = TilemapTexture::Vector(tilemap.textures);
        array_texture_loader.add(TilemapArrayTexture {
            texture: texture_vec.clone(),
            tile_size,
            format: TextureFormat::Bc1RgbaUnormSrgb,
            ..default()
        });

        commands.entity(tilemap.entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            map_type,
            size: tilemap_size,
            storage: tilemap.tile_storage,
            tile_size,
            transform,
            texture: texture_vec,
            ..default()
        });
    }
}
