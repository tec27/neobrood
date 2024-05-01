use bevy::{math::I16Vec2, prelude::*, utils::smallvec::SmallVec};
use std::ops::Index;

use crate::{
    gamedata::{
        AnimFrameCount, BwImage, BwSoundRange, BwSprite, Construct, ConstructFlags,
        ConstructTypeId, Flingy, LoadingAnimBundle, RenderStyle, CONSTRUCTS, IMAGES, SPRITES,
    },
    maps::position::Position,
    math::{bounds::IBounds, FixedPoint},
    races::Race,
    render::ysort::YSort,
    settings::GameSettings,
};

use super::{
    build_time::UnderConstruction,
    facing_direction::{apply_facing_to_images, FacingDirection},
    health::Health,
    iscripts::IscriptController,
};

pub fn plugin(app: &mut App) {
    app.register_type::<ConstructTypeId>()
        .register_type::<ConstructSprite>()
        .register_type::<OwnedConstruct>()
        .register_type::<ConstructImageOrder>()
        .add_systems(
            Update,
            (update_construct_image_order, update_construct_image_frames)
                .chain()
                .after(apply_facing_to_images),
        );
}

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

    /// Returns whether this construct type is a "neutral unit". These will still be spawned even in
    /// non-UMS games when placed with a map editor.
    pub fn is_neutral(&self) -> bool {
        match self {
            // Is a mineral field
            ConstructTypeId::ResourceMineralField1
            | ConstructTypeId::ResourceMineralField2
            | ConstructTypeId::ResourceMineralField3 => true,
            // Or a vespene geyser
            ConstructTypeId::ResourceVespeneGeyser => true,
            // Or a critter
            ConstructTypeId::CritterBengalaas
            | ConstructTypeId::CritterKakaru
            | ConstructTypeId::CritterRagnasaur
            | ConstructTypeId::CritterRhynadon
            | ConstructTypeId::CritterScantid
            | ConstructTypeId::CritterUrsadon => true,
            _ => false,
        }
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

    /// The current "main image" of the sprite, usually the image specified in the [BwSprite] data,
    /// but can be changed via iscript. Ordering of insertions into `images` will usually depend on
    /// this image, and certain iscript operations use it directly. If a value is present, it should
    /// always be in `images` as well.
    main_image: Option<Entity>,
    pub images: SmallVec<[Entity; 4]>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Ordering for layers of images within a sprite.
pub enum ImageOrder {
    /// The image is placed on top of all other images.
    Top,
    /// The image is placed just above the specified image, or above the main image if no image is
    /// specified. This is the default.
    Above(Option<Entity>),
    /// The image is placed just below the specified image, or below the main image if no image is
    /// specified.
    Below(Option<Entity>),
    /// The image is placed below all other images.
    Bottom,
}

impl Default for ImageOrder {
    fn default() -> Self {
        ImageOrder::Above(None)
    }
}

impl ConstructSprite {
    pub fn def(&self) -> &'static BwSprite {
        &SPRITES[self.id as usize]
    }

    pub fn set_main_image(&mut self, image: Entity) {
        debug_assert!(self.images.contains(&image));
        self.main_image = Some(image);
    }

    pub fn main_image(&self) -> Option<Entity> {
        self.main_image
    }

    /// Add an image to this sprite using the specified ordering. If this is the first image to be
    /// added it will become the main image for the sprite.
    pub fn add_image(&mut self, image: Entity, order: ImageOrder) {
        if self.images.is_empty() {
            self.main_image = Some(image);
            self.images.push(image);
            return;
        }

        match order {
            ImageOrder::Top => self.images.insert(0, image),
            ImageOrder::Bottom => self.images.push(image),
            ImageOrder::Above(Some(ref above)) => {
                let i = self.images.iter().position(|i| i == above).unwrap_or(0);
                self.images.insert(i, image);
            }
            ImageOrder::Above(None) => {
                let i = self
                    .main_image
                    .and_then(|ref main_image| self.images.iter().position(|i| i == main_image))
                    .unwrap_or(0);
                self.images.insert(i, image);
            }
            ImageOrder::Below(Some(ref below)) => {
                let i = self
                    .images
                    .iter()
                    .position(|i| i == below)
                    .unwrap_or_else(|| self.images.len() - 1)
                    + 1;
                if i < self.images.len() {
                    self.images.insert(i, image);
                } else {
                    self.images.push(image);
                }
            }
            ImageOrder::Below(None) => {
                let i = self
                    .main_image
                    .and_then(|ref main_image| self.images.iter().position(|i| i == main_image))
                    .unwrap_or_else(|| self.images.len() - 1)
                    + 1;
                if i < self.images.len() {
                    self.images.insert(i, image);
                } else {
                    self.images.push(image);
                }
            }
        }
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
            sprite: ConstructSprite { id, ..default() },
            ..default()
        }
    }
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
    /// How to render this image (if any special handling is needed).
    pub render_style: Option<RenderStyle>,
}

impl ConstructImage {
    pub fn def(&self) -> &'static BwImage {
        &IMAGES[self.id as usize]
    }
}

/// A component that specifies this [ConstructImage]'s order within its parent sprite's images (0
/// being the top-most image).
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Default, Reflect)]
pub struct ConstructImageOrder(pub u8);

#[derive(Bundle, Default)]
pub struct ConstructImageBundle {
    pub image: ConstructImage,
    pub spatial: SpatialBundle,
    pub loading_anim: LoadingAnimBundle,
    pub iscript: IscriptController,
    pub image_order: ConstructImageOrder,
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

/// System that applies a sprite's image ordering to the individual images.
pub fn update_construct_image_order(
    q_sprites: Query<&ConstructSprite, Changed<ConstructSprite>>,
    mut q_images: Query<&mut ConstructImageOrder>,
) {
    for sprite in q_sprites.iter() {
        let mut images = q_images.iter_many_mut(sprite.images.iter());
        let mut i = 0u8;
        while let Some(mut image_order) = images.fetch_next() {
            image_order.0 = i;
            i += 1;
        }
    }
}

/// System to update the current texture atlas frame for changed [ConstructImage] components.
pub fn update_construct_image_frames(
    mut query: Query<
        (
            &mut ConstructImage,
            &ConstructImageOrder,
            &mut TextureAtlas,
            &mut Sprite,
            &mut Transform,
            Option<&AnimFrameCount>,
        ),
        Or<(Changed<ConstructImage>, Changed<AnimFrameCount>)>,
    >,
    settings: Res<GameSettings>,
) {
    let tile_scale = settings.asset_quality.scale();
    for (mut image, image_order, mut atlas, mut sprite, mut transform, frame_count) in
        query.iter_mut()
    {
        atlas.index = (image.frame_base + image.frame_offset) as usize;
        if let Some(frame_count) = frame_count {
            if atlas.index >= frame_count.0 {
                image.frame_base = 0;
                image.frame_offset = 0;
                atlas.index = 0;
            }
        }
        sprite.flip_x = image.flip_x;
        transform.translation = (Vec2::new(image.offset.x as f32, -image.offset.y as f32)
            * tile_scale)
            .extend(-(image_order.0 as f32 / 1000.0));

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
