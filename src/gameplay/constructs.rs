use bevy::prelude::*;
use std::ops::Index;

use crate::{
    gamedata::{Construct, ConstructTypeId, Flingy, CONSTRUCTS},
    maps::position::Position,
    math::bounds::IBounds,
    races::Race,
    render::ysort::YSort,
};

impl Race {
    /// Returns the [Construct] for this race's starting building.
    pub fn hq_building(&self) -> &'static Construct {
        match self {
            Race::Protoss => &CONSTRUCTS[ConstructTypeId::ProtossNexus],
            Race::Terran => &CONSTRUCTS[ConstructTypeId::TerranCommandCenter],
            Race::Zerg => &CONSTRUCTS[ConstructTypeId::ZergHatchery],
        }
    }

    /// Returns the [Construct] for this race's worker unit.
    pub fn worker(&self) -> &'static Construct {
        match self {
            Race::Protoss => &CONSTRUCTS[ConstructTypeId::ProtossProbe],
            Race::Terran => &CONSTRUCTS[ConstructTypeId::TerranScv],
            Race::Zerg => &CONSTRUCTS[ConstructTypeId::ZergDrone],
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
}

/// Component that specifies a [Construct]'s owner (via a player number).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OwnedConstruct(pub u8);

#[derive(Bundle)]
pub struct ConstructBundle {
    pub spatial: SpatialBundle,
    pub construct_type: ConstructTypeId,
    pub position: Position,
    pub ysort: YSort,
}

impl Default for ConstructBundle {
    fn default() -> Self {
        Self {
            spatial: Default::default(),
            position: Default::default(),
            construct_type: Default::default(),
            ysort: YSort(2.0),
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
