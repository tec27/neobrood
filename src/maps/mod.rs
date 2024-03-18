use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    constructs::{ConstructTypeId, OwnedConstruct},
    gamedata::{BwGameData, LoadingAnim, CONSTRUCTS, SPRITES},
    maps::game_map::GameMapBundle,
    render::ysort::YSort,
    states::AppState,
};

use self::{
    asset::{MapAsset, MapAssetLoader},
    game_map::GameMap,
};

mod asset;
pub mod game_map;
mod tileset;

pub struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_asset::<MapAsset>()
            .init_asset_loader::<MapAssetLoader>()
            .init_resource::<CurrentMap>()
            // TODO(tec27): Maybe this should be handled as a requirement of PreGame and we
            // guarantee that exactly one map is loaded for InGame?
            .add_systems(Update, map_init.run_if(in_state(AppState::PreGame)))
            .add_systems(OnExit(AppState::InGame), map_cleanup);
    }
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
    game_map_query: Query<Entity, With<GameMap>>,
    mut next_state: ResMut<NextState<AppState>>,
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
    let map_entity = commands.spawn(GameMapBundle::default()).id();
    create_tilemap(&mut commands, map, &array_texture_loader, map_entity);
    create_map_sprites(&mut commands, map, map_entity);
    create_placed_units(&mut commands, map, map_entity);

    // TODO(tec27): This should probably be done in response to this stuff we just created being
    // ready? (i.e. it should wait for all the LoadingAnims that get added to be loaded)
    next_state.set(AppState::InGame);
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

fn create_map_sprites(commands: &mut Commands, map: &MapAsset, map_entity: Entity) {
    info!(
        "Creating map sprites, map has {} sprites",
        map.sprites.len()
    );

    // Used for inverting y-coords
    let max_height = (map.height - 1) as f32;

    for (i, sprite) in map.sprites.iter().enumerate() {
        let Some(s) = SPRITES.get(sprite.id as usize) else {
            warn!(
                "Encountered Sprite {} which isn't a valid ID, skipping",
                sprite.id
            );
            continue;
        };

        commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    // TODO(tec27): Need to base this on the map's tile size, would probably be
                    // better to write something that keeps track of this sprite in map coords and
                    // manages this transform value
                    ((sprite.x as f32) / 32.0 - map.width as f32 / 2.0) * 64.0 - 32.0,
                    ((max_height - (sprite.y as f32) / 32.0) - map.height as f32 / 2.0) * 64.0
                        + 32.0,
                    1.0,
                )),
                ..default()
            })
            .insert(YSort(2.0))
            .insert(Name::new(format!("Sprite #{i}")))
            .with_children(|builder| {
                builder.spawn(LoadingAnim::new(s.image_id));
            })
            .set_parent(map_entity);
    }
}

fn create_placed_units(commands: &mut Commands, map: &MapAsset, map_entity: Entity) {
    info!(
        "Creating placed units, map has {} placed units",
        map.placed_units.len()
    );

    // Used for inverting y-coords
    let max_height = (map.height - 1) as f32;

    for unit in map.placed_units.iter() {
        let Some(construct) = CONSTRUCTS.get(unit.unit_id as usize) else {
            warn!(
                "Encountered Unit {} which isn't a valid ID, skipping",
                unit.unit_id
            );
            continue;
        };

        let image_id = construct.flingy().sprite().image_id;

        let entity = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(
                        // TODO(tec27): Need to base this on the map's tile size, would probably be
                        // better to write something that keeps track of this sprite in map coords and
                        // manages this transform value
                        ((unit.x as f32) / 32.0 - map.width as f32 / 2.0) * 64.0 - 32.0,
                        ((max_height - (unit.y as f32) / 32.0) - map.height as f32 / 2.0) * 64.0
                            + 32.0,
                        1.0,
                    )),
                    ..default()
                },
                ConstructTypeId::from(unit.unit_id),
                YSort(2.0),
                Name::new(format!("Unit #{}", unit.unit_id)),
            ))
            .with_children(|builder| {
                builder.spawn(LoadingAnim::new(image_id));
            })
            .set_parent(map_entity)
            .id();
        if let Some(owner) = unit.owner {
            commands.entity(entity).insert(OwnedConstruct(owner));
        }
    }
}
