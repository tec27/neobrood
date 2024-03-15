use bevy::reflect::Reflect;

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum Race {
    #[default] // sorry Artosis :(
    Protoss,
    Terran,
    Zerg,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RaceSelection {
    #[default]
    Random,
    Protoss,
    Terran,
    Zerg,
}

impl From<Race> for RaceSelection {
    fn from(value: Race) -> Self {
        match value {
            Race::Protoss => RaceSelection::Protoss,
            Race::Terran => RaceSelection::Terran,
            Race::Zerg => RaceSelection::Zerg,
        }
    }
}
