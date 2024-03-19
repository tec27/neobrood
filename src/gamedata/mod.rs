use bevy::{asset::LoadState, prelude::*, sprite::Anchor, transform::TransformSystem};

use crate::{
    asset_packs::{AssetPack, AssetQuality},
    states::AppState,
};

use self::{
    anim::{AnimAsset, AnimAssetLoader},
    rel::{RelAsset, RelAssetLoader},
    tbl::{TblAsset, TblAssetLoader},
};

pub mod anim;
mod construct;
mod flingy;
mod generated;
mod image;
pub mod rel;
mod sprite;
pub mod tbl;

pub use construct::*;
pub use flingy::Flingy;
pub use generated::flingy::FLINGIES;
pub use generated::image::IMAGES;
pub use generated::sprite::SPRITES;
pub use generated::unit::CONSTRUCTS;
pub use image::*;
pub use sprite::*;

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TblAsset>()
            .init_asset_loader::<TblAssetLoader>()
            .init_asset::<RelAsset>()
            .init_asset_loader::<RelAssetLoader>()
            .init_asset::<AnimAsset>()
            .init_asset_loader::<AnimAssetLoader>()
            .register_type::<LoadingAnim>()
            .register_type::<AnimOffsets>()
            .add_systems(OnEnter(AppState::PreGame), load_game_data)
            .add_systems(
                Update,
                check_game_data_load.run_if(in_state(AppState::PreGame)),
            )
            // TODO(tec27): Maybe make a separate schedule for this. This one is public and in the
            // correct spot (and this is a very similar usecase) but it's not mentioned much in the
            // docs so it feels a bit iffy that it will exist forever? Unsure
            .add_systems(SpawnScene, init_loaded_anims)
            .add_systems(
                PostUpdate,
                update_anim_offsets
                    .before(TransformSystem::TransformPropagate)
                    .run_if(in_state(AppState::InGame)),
            );
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

    pub relations: Handle<RelAsset>,
}

#[derive(Resource, Debug)]
pub struct BwGameData {
    pub image_paths: TblAsset,
    pub strings: TblAsset,
    pub relations: RelAsset,
}

#[derive(Component, Debug, Default, Reflect)]
pub struct LoadingAnim {
    pub anim_id: u16,
    handle: Option<Handle<AnimAsset>>,
}

impl LoadingAnim {
    pub fn new(anim_id: u16) -> Self {
        Self {
            anim_id,
            handle: None,
        }
    }
}

fn load_game_data(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    game_data: Option<Res<BwGameData>>,
    loading_game_data: Option<Res<LoadingBwGameDataHandles>>,
) {
    if game_data.is_some() {
        // We already have game data loaded, so we can proceed to the next state
        info!("Game data is already loaded, proceeding to InGame state...");
        next_state.set(AppState::InGame);
        return;
    }
    if loading_game_data.is_some() {
        // We're already in the process of loading game data, so we can let that play out
        return;
    }

    let image_paths = asset_server.load("casc-extracted/arr/images.tbl");
    let strings = asset_server.load("casc-extracted/rez/stat_txt.tbl");

    let relations = asset_server.load("casc-extracted/images.rel");

    commands.insert_resource(LoadingBwGameDataHandles {
        image_paths,
        strings,
        relations,
    });
}

fn check_game_data_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handles: Option<Res<LoadingBwGameDataHandles>>,
    tbl_assets: Res<Assets<TblAsset>>,
    rel_assets: Res<Assets<RelAsset>>,
) {
    let Some(handles) = handles else {
        // No game data is currently loading, so there's nothing for us to do
        return;
    };

    // TODO(tec27): Handle load failures
    if asset_server.is_loaded_with_dependencies(&handles.image_paths)
        && asset_server.is_loaded_with_dependencies(&handles.strings)
        && asset_server.is_loaded_with_dependencies(&handles.relations)
    {
        commands.remove_resource::<LoadingBwGameDataHandles>();

        commands.insert_resource(BwGameData {
            image_paths: tbl_assets.get(&handles.image_paths).unwrap().clone(),
            strings: tbl_assets.get(&handles.strings).unwrap().clone(),
            relations: rel_assets.get(&handles.relations).unwrap().clone(),
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
    mut query: Query<(Entity, &mut LoadingAnim)>,
    anim_assets: Res<Assets<AnimAsset>>,
    game_data: Option<Res<BwGameData>>,
    asset_server: Res<AssetServer>,
) {
    let Some(game_data) = game_data else {
        // We don't have game data yet, so we can't do anything
        return;
    };

    for (entity, mut loading_anim) in &mut query {
        let Some(ref handle) = loading_anim.handle else {
            let relation = game_data
                .relations
                .entries
                .get(loading_anim.anim_id as usize)
                .copied()
                .unwrap_or_default();
            let id = if relation.is_image_reference() && relation.ref_image.is_some() {
                relation.ref_image.unwrap() as u16
            } else {
                loading_anim.anim_id
            };

            loading_anim.handle = Some(asset_server.load(format!(
                "casc-extracted/{}{}anim/main_{:03}.anim",
                // TODO(tec27): Make configurable in settings
                AssetQuality::High.asset_path(),
                AssetPack::Standard.asset_path(),
                id
            )));
            continue;
        };

        if let Some(anim) = anim_assets.get(handle) {
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
        } else if asset_server.load_state(handle) == LoadState::Failed {
            // TODO(tec27): Show a dialog or something instead?
            panic!(
                "Failed to load anim asset for anim_id {}",
                loading_anim.anim_id
            );
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
