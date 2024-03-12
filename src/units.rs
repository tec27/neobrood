use bevy::ecs::component::Component;

use crate::races::Race;

/// Specifies the type (e.g. class) of a unit (i.e. marine, zergling, mineral field, etc.).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitType(pub u16);

/// The unit type for start locations on the map.
pub const START_LOCATION: UnitType = UnitType(214);

// Protoss Buildings
pub const NEXUS: UnitType = UnitType(154);
// Terran Buildings
pub const COMMAND_CENTER: UnitType = UnitType(106);
// Zerg Buildings
pub const HATCHERY: UnitType = UnitType(131);

/// Returns the unit type for the specified race's starting building.
pub fn hq_building(race: Race) -> UnitType {
    match race {
        Race::Protoss => NEXUS,
        Race::Terran => COMMAND_CENTER,
        Race::Zerg => HATCHERY,
    }
}
