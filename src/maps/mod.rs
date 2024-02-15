use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    gamedata::BwGameData,
    maps::{game_map::GameMapBundle, sprites::create_placed_units},
};

use self::{
    asset::{MapAsset, MapAssetLoader},
    sprites::create_map_sprites,
};

mod asset;
pub mod game_map;
mod sprites;
mod tileset;

pub struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_asset::<MapAsset>()
            .init_asset_loader::<MapAssetLoader>()
            .init_resource::<CurrentMap>()
            .add_systems(Update, map_init);
    }
}

#[derive(Resource, Default)]
pub struct CurrentMap {
    pub handle: Handle<MapAsset>,
}

fn map_init(
    mut commands: Commands,
    mut asset_events: EventReader<AssetEvent<MapAsset>>,
    game_data: Option<Res<BwGameData>>,
    current_map: Res<CurrentMap>,
    map_assets: Res<Assets<MapAsset>>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    let Some(game_data) = game_data else {
        // Wait for game data to load before we process any maps, since we need it to properly deal
        // with sprites and units
        return;
    };

    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            if *id == current_map.handle.id() {
                info!("Map loaded!");
                let map = map_assets.get(*id).unwrap();
                let map_entity = commands.spawn(GameMapBundle::default()).id();
                create_tilemap(&mut commands, map, &array_texture_loader, map_entity);
                create_map_sprites(&mut commands, map, map_entity, &game_data);
                create_placed_units(&mut commands, map, map_entity);
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
    map_entity: Entity,
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

        commands.entity(map_entity).add_child(tilemap.entity);
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
