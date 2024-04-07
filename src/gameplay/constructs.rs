use bevy::prelude::*;
use std::ops::Index;

use crate::{
    gamedata::{
        BwImage, BwSprite, Construct, ConstructFlags, ConstructTypeId, Flingy, LoadingAnimBundle,
        CONSTRUCTS, IMAGES, SPRITES,
    },
    maps::position::Position,
    math::{bounds::IBounds, FixedPoint},
    races::Race,
    render::ysort::YSort,
};

use super::{build_time::UnderConstruction, facing_direction::FacingDirection, health::Health};

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

/// Component that specifies an entity is an image for a [ConstructSprite].
#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
pub struct ConstructImage {
    /// The ID of the image this entity maps to. Can be looked up in [IMAGES].
    pub id: u16,
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
}

impl ConstructImageBundle {
    pub fn new(id: u16) -> Self {
        Self {
            image: ConstructImage { id },
            loading_anim: LoadingAnimBundle::new(id),
            ..default()
        }
    }
}
