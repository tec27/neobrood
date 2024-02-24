use bevy::{prelude::*, sprite::Anchor};

use crate::render::ysort::YSort;

use self::{
    anim::{AnimAsset, AnimAssetLoader},
    dat::{DatAsset, DatAssetLoader, FlingyData, ImageData, SpriteData, UnitData},
    tbl::{TblAsset, TblAssetLoader},
};

pub mod anim;
pub mod dat;
pub mod tbl;

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TblAsset>()
            .init_asset_loader::<TblAssetLoader>()
            .init_asset::<DatAsset>()
            .init_asset_loader::<DatAssetLoader>()
            .init_asset::<AnimAsset>()
            .init_asset_loader::<AnimAssetLoader>()
            .register_type::<LoadingAnim>()
            .register_type::<AnimOffsets>()
            .add_systems(Startup, load_game_data)
            .add_systems(Update, (check_game_data_load, init_loaded_anims))
            .add_systems(PostUpdate, update_anim_offsets);
    }
}

/// Resource that tracks which handles we have for currently loading BW game data files. When they
/// have been completely loaded, we will extract their underlying data into a [BwGameData] resource
/// and discard this resource.
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct LoadingBwGameDataHandles {
    pub image_paths: Handle<TblAsset>,
    pub strings: Handle<TblAsset>,

    pub flingy: Handle<DatAsset>,
    pub images: Handle<DatAsset>,
    pub sprites: Handle<DatAsset>,
    pub units: Handle<DatAsset>,
}

#[derive(Resource, Debug)]
pub struct BwGameData {
    pub image_paths: TblAsset,
    pub strings: TblAsset,

    pub flingy: FlingyData,
    pub images: ImageData,
    pub sprites: SpriteData,
    pub units: UnitData,
}

#[derive(Component, Debug, Default, Reflect)]
pub struct LoadingAnim {
    pub handle: Handle<AnimAsset>,
}

fn load_game_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_paths = asset_server.load("casc-extracted/arr/images.tbl");
    let strings = asset_server.load("casc-extracted/rez/stat_txt.tbl");

    let flingy = asset_server.load("casc-extracted/arr/flingy.dat");
    let images = asset_server.load("casc-extracted/arr/images.dat");
    let sprites = asset_server.load("casc-extracted/arr/sprites.dat");
    let units = asset_server.load("casc-extracted/arr/units.dat");

    commands.insert_resource(LoadingBwGameDataHandles {
        image_paths,
        strings,

        flingy,
        images,
        sprites,
        units,
    });
}

fn check_game_data_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handles: Option<Res<LoadingBwGameDataHandles>>,
    tbl_assets: Res<Assets<TblAsset>>,
    dat_assets: Res<Assets<DatAsset>>,
) {
    let Some(handles) = handles else {
        // No game data is currently loading, so there's nothing for us to do
        return;
    };

    // TODO(tec27): Handle load failures
    if asset_server.is_loaded_with_dependencies(&handles.image_paths)
        && asset_server.is_loaded_with_dependencies(&handles.strings)
        && asset_server.is_loaded_with_dependencies(&handles.flingy)
        && asset_server.is_loaded_with_dependencies(&handles.images)
        && asset_server.is_loaded_with_dependencies(&handles.sprites)
        && asset_server.is_loaded_with_dependencies(&handles.units)
    {
        commands.remove_resource::<LoadingBwGameDataHandles>();

        commands.insert_resource(BwGameData {
            image_paths: tbl_assets.get(&handles.image_paths).unwrap().clone(),
            strings: tbl_assets.get(&handles.strings).unwrap().clone(),

            flingy: dat_assets
                .get(&handles.flingy)
                .unwrap()
                .try_into()
                .expect("Failed to convert flingy DatAsset to underlying data"),
            images: dat_assets
                .get(&handles.images)
                .unwrap()
                .try_into()
                .expect("Failed to convert images DatAsset to underlying data"),
            sprites: dat_assets
                .get(&handles.sprites)
                .unwrap()
                .try_into()
                .expect("Failed to convert sprites DatAsset to underlying data"),
            units: dat_assets
                .get(&handles.units)
                .unwrap()
                .try_into()
                .expect("Failed to convert units DatAsset to underlying data"),
        });

        info!("BW game data has been loaded!");
    }
}

#[derive(Component, Debug, Default, Reflect)]
pub struct AnimOffsets {
    pub offsets: Vec<Anchor>,
}

fn init_loaded_anims(
    mut commands: Commands,
    query: Query<(Entity, &LoadingAnim)>,
    anim_assets: Res<Assets<AnimAsset>>,
) {
    for (entity, loading_anim) in query.iter() {
        if let Some(anim) = anim_assets.get(&loading_anim.handle) {
            commands.entity(entity).remove::<LoadingAnim>().insert((
                SpriteSheetBundle {
                    texture: anim.layers.get("diffuse").cloned().unwrap_or_default(),
                    atlas: anim.layout.clone().into(),
                    ..default()
                },
                AnimOffsets {
                    offsets: anim.offsets.clone(),
                },
            ));
        }
    }
}

fn update_anim_offsets(
    mut query: Query<
        (&AnimOffsets, &TextureAtlas, &mut Sprite),
        Or<(Changed<AnimOffsets>, Changed<TextureAtlas>)>,
    >,
) {
    for (offsets, atlas, mut sprite) in query.iter_mut() {
        sprite.anchor = offsets
            .offsets
            .get(atlas.index)
            .copied()
            .unwrap_or_default()
    }
}
