use bevy::ecs::component::Component;
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::races::Race;

/// Specifies the type (e.g. class) of a unit (i.e. marine, zergling, mineral field, etc.).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum UnitType {
    TerranCommandCenter = 106,
    ZergHatchery = 131,
    ProtossNexus = 154,
    StartLocation = 214,
    #[num_enum(catch_all)]
    Unknown(u16),
}

/// Returns the unit type for the specified race's starting building.
pub fn hq_building(race: Race) -> UnitType {
    match race {
        Race::Protoss => UnitType::ProtossNexus,
        Race::Terran => UnitType::TerranCommandCenter,
        Race::Zerg => UnitType::ZergHatchery,
    }
}

/// Component that specifies a unit's owner (via a player number).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OwnedUnit(pub u8);
