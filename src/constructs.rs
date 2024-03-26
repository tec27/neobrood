use bevy::prelude::*;
use std::ops::Index;

use crate::{
    gamedata::{Construct, ConstructTypeId, CONSTRUCTS},
    maps::position::Position,
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

const fn max_construct_size() -> i32 {
    let mut max_size = 0;
    let mut i = 0;
    while i < CONSTRUCTS.len() {
        let size = CONSTRUCTS[i].bounds.size();
        if max_size < size.x {
            max_size = size.x;
        }
        if max_size < size.y {
            max_size = size.y;
        }
        i += 1;
    }

    return max_size;
}

/// The maximum size of any [Construct] (width or height) in logical pixels. This is intended to be
/// useful when deciding how to size spatial indexes or other data structures that need to deal
/// with positional data and overlaps.
pub const MAX_CONSTRUCT_SIZE: i32 = max_construct_size();
