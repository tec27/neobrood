use bevy::{asset::LoadState, prelude::*, sprite::Anchor, transform::TransformSystem};

use crate::{
    gameplay::constructs::ConstructImage,
    settings::{AssetPack, GameSettings},
    states::AppState,
};

use self::{
    anim::{AnimAsset, AnimAssetLoader},
    lo::{LoAsset, LoAssetLoader},
    rel::{RelAsset, RelAssetLoader},
    tbl::{TblAsset, TblAssetLoader},
};

pub mod anim;
mod construct;
mod flingy;
mod generated;
mod image;
mod iscript;
pub mod lo;
pub mod rel;
mod sound;
mod sprite;
pub mod tbl;

pub use construct::*;
pub use flingy::*;
pub use generated::flingy::FLINGIES;
pub use generated::image::IMAGES;
pub use generated::iscript::ISCRIPTS;
pub use generated::iscript::ISCRIPT_ANIMS;
pub use generated::sound::SOUNDS;
pub use generated::sprite::SPRITES;
pub use generated::unit::CONSTRUCTS;
pub use image::*;
pub use iscript::*;
pub use sound::*;
pub use sprite::*;

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimAsset>()
            .init_asset_loader::<AnimAssetLoader>()
            .init_asset::<LoAsset>()
            .init_asset_loader::<LoAssetLoader>()
            .init_asset::<RelAsset>()
            .init_asset_loader::<RelAssetLoader>()
            .init_asset::<TblAsset>()
            .init_asset_loader::<TblAssetLoader>()
            .register_type::<LoadingAnim>()
            .register_type::<AnimOffsets>()
            .register_type::<AnimFrameCount>()
            .register_type::<SpecialOverlay>()
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

        app.world_mut()
            .register_component_hooks::<LoadingAnim>()
            .on_add(|mut world, entity, _component_id| {
                let Some(game_data) = world.get_resource::<BwGameData>() else {
                    error!("LoadingAnim added before game data has been loaded");
                    return;
                };
                let Some(settings) = world.get_resource::<GameSettings>() else {
                    error!("LoadingAnim added before GameSettings added");
                    return;
                };

                let loading_anim = world.get::<LoadingAnim>(entity).unwrap();
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

                let asset_pack = if id == START_LOCATION_ID {
                    AssetPack::Standard
                } else {
                    settings.asset_pack
                };

                let asset_server = world.get_resource::<AssetServer>().unwrap();
                let handle: Handle<AnimAsset> = asset_server.load(format!(
                    "casc-extracted/{}anim/{}main_{:03}.anim",
                    settings.asset_quality.asset_path(),
                    asset_pack.asset_path(),
                    id
                ));

                let Some(image_def) = IMAGES.get(loading_anim.anim_id as usize) else {
                    warn!(
                        "No image definition found for anim_id {}",
                        loading_anim.anim_id
                    );
                    world.commands().entity(entity).insert(handle);
                    return;
                };
                // TODO(tec27): implement other overlay types
                if let Some(special_overlay) = image_def
                    .special_overlay
                    .and_then(|o| game_data.image_paths.get(o.get() as usize))
                {
                    let handle: Handle<LoAsset> =
                        asset_server.load(format!("casc-extracted\\unit\\{}", special_overlay));
                    world
                        .commands()
                        .entity(entity)
                        .insert(SpecialOverlay(handle));
                }

                world.commands().entity(entity).insert(handle);
            });
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
struct LoadingAnim {
    anim_id: u16,
}

#[derive(Bundle, Default)]
pub struct LoadingAnimBundle {
    loading_anim: LoadingAnim,
    // NOTE(tec27): We need to insert the TextureAtlas + Sprite immediately so that any updates to
    // the frame before the anim loads will still be reflected
    pub texture_atlas: TextureAtlas,
    pub sprite: Sprite,
}

impl LoadingAnimBundle {
    #[inline]
    pub fn new(anim_id: u16) -> Self {
        Self {
            loading_anim: LoadingAnim { anim_id },
            ..default()
        }
    }
}

fn load_game_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_data: Option<Res<BwGameData>>,
    loading_game_data: Option<Res<LoadingBwGameDataHandles>>,
) {
    if game_data.is_some() {
        // We already have game data loaded, nothing to do here
        info!("Game data is already loaded");
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

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct AnimFrameCount(pub usize);

impl Default for AnimFrameCount {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct SpecialOverlay(pub Handle<LoAsset>);

impl From<&SpecialOverlay> for AssetId<LoAsset> {
    fn from(overlay: &SpecialOverlay) -> Self {
        overlay.0.id()
    }
}

/// Image ID of the Start Location graphic, which only exists in the standard asset pack.
const START_LOCATION_ID: u16 = 588;

/// Bundle for an anim that has already had its assets loaded. Any entities that use this bundle
/// should have all the components from [SpriteSheetBundle] to render correctly.
#[derive(Bundle, Default)]
pub struct PreloadedAnimBundle {
    pub texture: Handle<Image>,
    pub atlas: TextureAtlas,
    pub anim_offsets: AnimOffsets,
    pub frame_count: AnimFrameCount,
}

impl PreloadedAnimBundle {
    pub fn for_asset(asset: &AnimAsset, index: usize) -> Self {
        Self {
            texture: asset.layers.get("diffuse").cloned().unwrap_or_default(),
            atlas: TextureAtlas {
                layout: asset.layout.clone(),
                index,
            },
            anim_offsets: AnimOffsets {
                offsets: asset.offsets.clone(),
            },
            frame_count: AnimFrameCount(asset.frame_count),
        }
    }
}

fn init_loaded_anims(
    mut commands: Commands,
    mut query: Query<(Entity, &LoadingAnim, &Handle<AnimAsset>, &TextureAtlas)>,
    anim_assets: Res<Assets<AnimAsset>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, loading_anim, handle, atlas) in &mut query {
        if let Some(anim) = anim_assets.get(handle) {
            commands
                .entity(entity)
                .remove::<LoadingAnim>()
                .insert(PreloadedAnimBundle::for_asset(anim, atlas.index));
        } else if let LoadState::Failed(error) = asset_server.load_state(handle) {
            // TODO(tec27): Show a dialog or something instead?
            panic!(
                "Failed to load anim asset for anim_id {}: {error:?}",
                loading_anim.anim_id
            );
        }
    }
}

fn flip_anchor_x(anchor: &Anchor) -> Anchor {
    match anchor {
        Anchor::TopLeft => Anchor::TopRight,
        Anchor::TopCenter => Anchor::TopCenter,
        Anchor::TopRight => Anchor::TopLeft,
        Anchor::CenterLeft => Anchor::CenterRight,
        Anchor::Center => Anchor::Center,
        Anchor::CenterRight => Anchor::CenterLeft,
        Anchor::BottomLeft => Anchor::BottomRight,
        Anchor::BottomCenter => Anchor::BottomCenter,
        Anchor::BottomRight => Anchor::BottomLeft,
        Anchor::Custom(p) => Anchor::Custom(Vec2::new(-p.x, p.y)),
    }
}

fn update_anim_offsets(
    mut query: Query<
        (
            &AnimOffsets,
            &TextureAtlas,
            &mut Sprite,
            Option<&ConstructImage>,
        ),
        Or<(
            Changed<AnimOffsets>,
            Changed<TextureAtlas>,
            Changed<ConstructImage>,
        )>,
    >,
) {
    for (offsets, atlas, mut sprite, image) in query.iter_mut() {
        let flip_x = image.map_or(false, |i| i.flip_x);
        sprite.anchor = offsets
            .offsets
            .get(atlas.index)
            .map(|anchor| {
                if flip_x {
                    flip_anchor_x(anchor)
                } else {
                    *anchor
                }
            })
            .unwrap_or_default()
    }
}
