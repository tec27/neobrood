/// Specifies the type (e.g. class) of a unit (i.e. marine, zergling, mineral field, etc.).
pub struct UnitType(pub u16);

/// The unit type for start locations on the map.
pub const START_LOCATION: UnitType = UnitType(214);
