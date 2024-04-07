use std::path::Path;

use bevy::render::render_resource::TextureFormat;
use bevy::utils::HashMap;
use bevy::{prelude::*, transform::TransformSystem};
use bevy_ecs_tilemap::prelude::*;

use crate::gameplay::constructs::{
    ConstructBundle, ConstructImageBundle, ConstructSpriteBundle, OwnedConstruct,
};
use crate::maps::game_map::GameMapTerrain;
use crate::settings::GameSettings;
use crate::{
    gamedata::{BwGameData, CONSTRUCTS, SPRITES},
    maps::game_map::{GameMapBundle, GameMapSize},
    render::ysort::YSort,
    states::AppState,
};
use asset::{MapAsset, MapAssetLoader};
use game_map::GameMap;
use position::apply_position_to_transform;
use position::Position;

mod asset;
pub mod game_map;
pub mod position;
mod tileset;

pub use asset::MapAssetSettings;

pub struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_asset::<MapAsset>()
            .init_asset_loader::<MapAssetLoader>()
            .init_resource::<CurrentMap>()
            .register_type::<GameMap>()
            .register_type::<GameMapSize>()
            .register_type::<Position>()
            // TODO(tec27): Maybe this should be handled as a requirement of PreGame and we
            // guarantee that exactly one map is loaded for InGame?
            .add_systems(Update, map_init.run_if(in_state(AppState::PreGame)))
            .add_systems(OnExit(AppState::InGame), map_cleanup)
            .add_systems(
                PostUpdate,
                apply_position_to_transform.before(TransformSystem::TransformPropagate),
            );
    }
}

pub fn load_map(
    path: &Path,
    current_map: &mut ResMut<CurrentMap>,
    next_state: &mut ResMut<NextState<AppState>>,
    asset_server: &Res<AssetServer>,
    settings: &Res<GameSettings>,
) {
    info!("Loading map: {}", path.to_string_lossy());
    next_state.set(AppState::PreGame);
    let quality = settings.asset_quality;
    let pack = settings.asset_pack;
    current_map.handle =
        asset_server.load_with_settings(path.to_owned(), move |s: &mut MapAssetSettings| {
            s.quality = quality;
            s.pack = pack;
        });
}

#[derive(Resource, Default)]
pub struct CurrentMap {
    pub handle: Handle<MapAsset>,
}

fn map_init(
    mut commands: Commands,
    game_data: Option<Res<BwGameData>>,
    current_map: Res<CurrentMap>,
    map_assets: Res<Assets<MapAsset>>,
    array_texture_loader: Res<ArrayTextureLoader>,
    settings: Res<GameSettings>,
    game_map_query: Query<Entity, With<GameMap>>,
) {
    if !game_map_query.is_empty() {
        // Map already initialized
        return;
    }
    if game_data.is_none() {
        return;
    }
    let Some(map) = map_assets.get(&current_map.handle) else {
        return;
    };

    info!("Map loaded!");
    let map_entity = commands
        .spawn((
            GameMapBundle {
                size: GameMapSize {
                    width: map.width,
                    height: map.height,
                },
                // TODO(tec27): Handle errors in this conversion properly
                terrain: GameMapTerrain::from_terrain_and_lookup(
                    &map.terrain,
                    &map.mega_tile_lookup,
                )
                .unwrap(),
                ..default()
            },
            Name::new(format!("GameMap - {}", map.name)),
        ))
        .id();
    create_tilemap(
        &mut commands,
        map,
        &array_texture_loader,
        &settings,
        map_entity,
    );
    create_map_sprites(&mut commands, map, map_entity);
    create_placed_units(&mut commands, map, map_entity);
}

fn map_cleanup(mut commands: Commands, maps: Query<Entity, With<GameMap>>) {
    for entity in maps.iter() {
        commands.entity(entity).despawn_recursive();
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
    settings: &Res<GameSettings>,
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

    let tile_size: TilemapTileSize = settings.asset_quality.tile_size().into();
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

fn create_map_sprites(commands: &mut Commands, map: &MapAsset, map_entity: Entity) {
    info!(
        "Creating map sprites, map has {} sprites",
        map.sprites.len()
    );

    for (i, sprite) in map.sprites.iter().enumerate() {
        let Some(s) = SPRITES.get(sprite.id as usize) else {
            warn!(
                "Encountered Sprite {} which isn't a valid ID, skipping",
                sprite.id
            );
            continue;
        };

        commands
            .spawn((
                SpatialBundle::default(),
                Position::new(sprite.x.into(), sprite.y.into()),
                YSort(2.0),
                Name::new(format!("Sprite #{i}")),
            ))
            .with_children(|builder| {
                builder
                    .spawn(ConstructSpriteBundle::new(s.id))
                    .with_children(|builder| {
                        builder.spawn(ConstructImageBundle::new(s.image_id));
                    });
            })
            .set_parent(map_entity);
    }
}

fn create_placed_units(commands: &mut Commands, map: &MapAsset, map_entity: Entity) {
    info!(
        "Creating placed units, map has {} placed units",
        map.placed_units.len()
    );

    for unit in map.placed_units.iter() {
        if CONSTRUCTS.get(unit.unit_id as usize).is_none() {
            warn!(
                "Encountered Unit {} which isn't a valid ID, skipping",
                unit.unit_id
            );
            continue;
        };

        let construct_type = unit.unit_id.into();

        let entity = commands
            .spawn((
                ConstructBundle {
                    construct_type,
                    position: Position::new(unit.x.into(), unit.y.into()),
                    ..default()
                },
                Name::new(format!("Unit #{} - {:?}", unit.unit_id, construct_type)),
            ))
            .with_children(|builder| {
                builder
                    .spawn(ConstructSpriteBundle::new(
                        construct_type.flingy().sprite_id,
                    ))
                    .with_children(|builder| {
                        builder.spawn(ConstructImageBundle::new(
                            construct_type.flingy().sprite().image_id,
                        ));
                    });
            })
            .set_parent(map_entity)
            .id();
        if let Some(owner) = unit.owner {
            commands.entity(entity).insert(OwnedConstruct(owner));
        }
    }
}
