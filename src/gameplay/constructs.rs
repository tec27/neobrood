use bevy::{math::I16Vec2, prelude::*};
use std::ops::Index;

use crate::{
    gamedata::{
        BwImage, BwSoundRange, BwSprite, Construct, ConstructFlags, ConstructTypeId, Flingy,
        LoadingAnimBundle, RenderStyle, CONSTRUCTS, IMAGES, SPRITES,
    },
    maps::position::Position,
    math::{bounds::IBounds, FixedPoint},
    races::Race,
    render::ysort::YSort,
    settings::GameSettings,
};

use super::{
    build_time::UnderConstruction, facing_direction::FacingDirection, health::Health,
    iscripts::IscriptController,
};

impl Race {
    /// Returns the [ConstructTypeId] for this race's starting building.
    pub fn hq_building(&self) -> ConstructTypeId {
        match self {
            Race::Protoss => ConstructTypeId::ProtossNexus,
            Race::Terran => ConstructTypeId::TerranCommandCenter,
            Race::Zerg => ConstructTypeId::ZergHatchery,
        }
    }

    /// Returns the [ConstructTypeId] for this race's worker unit.
    pub fn worker(&self) -> ConstructTypeId {
        match self {
            Race::Protoss => ConstructTypeId::ProtossProbe,
            Race::Terran => ConstructTypeId::TerranScv,
            Race::Zerg => ConstructTypeId::ZergDrone,
        }
    }
}

impl Index<ConstructTypeId> for [Construct; 228] {
    type Output = Construct;

    #[inline]
    fn index(&self, index: ConstructTypeId) -> &Self::Output {
        &self[u16::from(index) as usize]
    }
}

impl ConstructTypeId {
    /// Returns the [Construct] definition that matches this type ID.
    #[inline]
    pub fn def(&self) -> &Construct {
        &CONSTRUCTS[*self]
    }

    /// Returns the [Flingy] for this construct type.
    #[inline]
    pub fn flingy(&self) -> &'static Flingy {
        self.def().flingy()
    }

    /// Returns the bounds of this construct type.
    #[inline]
    pub fn bounds(&self) -> IBounds {
        self.def().bounds
    }

    /// Returns if this type of Construct is a unit.
    #[inline]
    pub fn is_unit(&self) -> bool {
        self.def().is_unit()
    }

    /// Returns if this type of Construct is a building.
    #[inline]
    pub fn is_building(&self) -> bool {
        self.def().is_building()
    }

    #[inline]
    pub fn max_health(&self) -> FixedPoint {
        self.def().hit_points
    }

    #[inline]
    pub fn shield_points(&self) -> Option<FixedPoint> {
        self.def().shield_points
    }

    #[inline]
    pub fn flags(&self) -> ConstructFlags {
        self.def().flags
    }

    #[inline]
    pub fn what_sounds(&self) -> Option<BwSoundRange> {
        self.def().what_sounds
    }
}

/// Component that specifies a [Construct]'s owner (via a player number).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub struct OwnedConstruct(pub u8);

#[derive(Bundle)]
pub struct ConstructBundle {
    pub spatial: SpatialBundle,
    pub construct_type: ConstructTypeId,
    pub position: Position,
    pub ysort: YSort,
    pub health: Health,
    pub under_construction: UnderConstruction,
    pub facing_direction: FacingDirection,
}

impl Default for ConstructBundle {
    fn default() -> Self {
        Self {
            spatial: Default::default(),
            position: Default::default(),
            construct_type: Default::default(),
            ysort: YSort(2.0),
            health: Default::default(),
            under_construction: Default::default(),
            facing_direction: Default::default(),
        }
    }
}

const fn max_construct_size() -> IVec2 {
    let mut max_size = IVec2::new(0, 0);
    let mut i = 0;
    while i < CONSTRUCTS.len() {
        let size = CONSTRUCTS[i].bounds.size();
        if max_size.x < size.x {
            max_size.x = size.x;
        }
        if max_size.y < size.y {
            max_size.y = size.y;
        }
        i += 1;
    }

    max_size
}

/// The maximum size of any [Construct] in logical pixels. This is intended to be useful when
/// deciding how to size spatial indexes or other data structures that need to deal with positional
/// data and overlaps.
pub const MAX_CONSTRUCT_SIZE: IVec2 = max_construct_size();

/// Component that specifies an entity is a sprite for a [Construct].
#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
pub struct ConstructSprite {
    /// The ID of the sprite this entity maps to. Can be looked up in [SPRITES].
    pub id: u16,
}

impl ConstructSprite {
    pub fn def(&self) -> &'static BwSprite {
        &SPRITES[self.id as usize]
    }
}

#[derive(Bundle, Default)]
pub struct ConstructSpriteBundle {
    pub sprite: ConstructSprite,
    pub spatial: SpatialBundle,
}

impl ConstructSpriteBundle {
    pub fn new(id: u16) -> Self {
        Self {
            sprite: ConstructSprite { id },
            ..default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Reflect)]
/// Ordering for layers of images within a sprite.
pub enum ImageOrder {
    /// The image is drawn on top of other images.
    Top,
    /// The image is drawn on top of any other images that are not `Top`. This is the default.
    #[default]
    Above,
    /// The image is only drawn on top of images that are `Bottom`.
    Below,
    /// The image is drawn below all other images.
    Bottom,
}

/// Component that specifies an entity is an image for a [ConstructSprite].
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
pub struct ConstructImage {
    /// The ID of the image this entity maps to. Can be looked up in [IMAGES].
    pub id: u16,
    /// The base frame index for this image, used alongside `frame_offset` to pick a frame from the
    /// texture atlas.
    pub frame_base: u16,
    /// The offset to apply to the base frame index to pick the actual frame from the texture atlas.
    pub frame_offset: u16,
    /// Whether to flip the image horizontally. This is generally used for half of the sprite
    /// rotations as one side is simply mirrored.
    pub flip_x: bool,
    /// The offset to apply from the sprite's location to this image, in logical pixels.
    pub offset: I16Vec2,
    /// The layering order for this image.
    pub order: ImageOrder,
    /// How to render this image (if any special handling is needed).
    pub render_style: Option<RenderStyle>,
}

impl ConstructImage {
    pub fn def(&self) -> &'static BwImage {
        &IMAGES[self.id as usize]
    }
}

#[derive(Bundle, Default)]
pub struct ConstructImageBundle {
    pub image: ConstructImage,
    pub spatial: SpatialBundle,
    pub loading_anim: LoadingAnimBundle,
    pub iscript: IscriptController,
}

impl ConstructImageBundle {
    pub fn new(id: u16) -> Self {
        let mut image = ConstructImage { id, ..default() };
        image.render_style = image.def().render_style;
        let iscript = IscriptController::for_image(&image);

        Self {
            image,
            iscript,
            loading_anim: LoadingAnimBundle::new(id),
            ..default()
        }
    }
}

const SHADOW_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.7);

/// System to update the current texture atlas frame for changed [ConstructImage] components.
pub fn update_construct_image_frames(
    mut query: Query<
        (
            &ConstructImage,
            &mut TextureAtlas,
            &mut Sprite,
            &mut Transform,
        ),
        Changed<ConstructImage>,
    >,
    settings: Res<GameSettings>,
) {
    let tile_scale = settings.asset_quality.scale();
    for (image, mut atlas, mut sprite, mut transform) in query.iter_mut() {
        atlas.index = (image.frame_base + image.frame_offset) as usize;
        sprite.flip_x = image.flip_x;
        transform.translation = (Vec2::new(image.offset.x as f32, -image.offset.y as f32)
            * tile_scale)
            .extend(match image.order {
                // TODO(tec27): This isn't really the correct behavior, this ordering is meant to
                // describe how to insert this image within the list of existing images, so we can't
                // easily calculate the correct z value at this point. Rework this somehow.
                ImageOrder::Top => 0.05,
                ImageOrder::Above => 0.01,
                ImageOrder::Below => -0.01,
                ImageOrder::Bottom => -0.05,
            });

        if image.render_style == Some(RenderStyle::Shadow) {
            // TODO(tec27): This isn't totally correct and doesn't deal with the "shadow stacking"
            // feature at all. I think shadows should always be 50% grey as well? Probably room for
            // creativity here.
            sprite.color = SHADOW_COLOR;
        } else if sprite.color == SHADOW_COLOR {
            sprite.color = Color::WHITE;
        }
        // TODO(tec27): Deal with other render styles, and probably avoid clobbering the sprite
        // color better?
    }
}
