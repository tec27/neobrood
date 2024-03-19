use std::ops::Index;

use bevy::ecs::component::Component;
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    gamedata::{Construct, CONSTRUCTS},
    races::Race,
};

/// Specifies the type (e.g. class) of a construct (i.e. marine, zergling, mineral field, etc.).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum ConstructTypeId {
    TerranCommandCenter = 106,
    ZergHatchery = 131,
    ProtossNexus = 154,
    StartLocation = 214,
    #[num_enum(catch_all)]
    Unknown(u16),
}

/// Returns the [Construct] for the specified race's starting building.
pub fn hq_building(race: Race) -> &'static Construct {
    match race {
        Race::Protoss => &CONSTRUCTS[ConstructTypeId::ProtossNexus],
        Race::Terran => &CONSTRUCTS[ConstructTypeId::TerranCommandCenter],
        Race::Zerg => &CONSTRUCTS[ConstructTypeId::ZergHatchery],
    }
}

impl Index<ConstructTypeId> for [Construct; 228] {
    type Output = Construct;

    #[inline]
    fn index(&self, index: ConstructTypeId) -> &Self::Output {
        &self[u16::from(index) as usize]
    }
}

/// Component that specifies a [Construct]'s owner (via a player number).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OwnedConstruct(pub u8);
