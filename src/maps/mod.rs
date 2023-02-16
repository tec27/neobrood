use bevy::asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_ecs_tilemap::prelude::*;
use broodmap::Chk;

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
    pub chk: Chk,
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
            let map = MapAsset { chk };
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
    asset_server: Res<AssetServer>,
) {
    for event in asset_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if *handle == current_map.handle {
                println!("Map loaded!");
                let map = map_assets.get(handle).unwrap();
                create_tilemap(&mut commands, &asset_server, map);
            }
        }
    }
}

fn create_tilemap(commands: &mut Commands, asset_server: &Res<AssetServer>, map: &MapAsset) {
    let map_size = TilemapSize {
        x: map.chk.width() as u32,
        y: map.chk.height() as u32,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let Ok(terrain) = map.chk.terrain_mega_tiles() else {
        // TODO(tec27): Probably we should fail during the asset load instead of here, maybe we
        // should just copy out the data we care about and not pass around the Chk?
        panic!("Failed to load terrain from map!");
    };

    let texture_handle: Handle<Image> = asset_server.load("dev-tile.png");

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let mega_tile = terrain[y as usize][x as usize];
            // Bevy coords start from the bottom-left, rather than top-left like the map data
            let mapped_y = map_size.y - 1 - y;
            let tile_pos = TilePos { x, y: mapped_y };

            // TODO(tec27): Use actual graphics, this is just to see that the tilemap is working
            let red = (mega_tile >> 8) as f32 / 255.0;
            let green = ((mega_tile >> 6) & 0xFF) as f32 / 255.0;
            let blue = (mega_tile & 0xFF) as f32 / 255.0;
            let color = Color::rgb(red, green, blue);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    color: color.into(),
                    texture_index: TileTextureIndex(0),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        texture: TilemapTexture::Single(texture_handle),
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
