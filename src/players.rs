use bevy::{ecs::component::Component, reflect::Reflect};

use crate::races::Race;

/// Component that specifies a player in the game. This is used for both human and AI players.
#[allow(unused)]
#[derive(Component, Debug, Default, Reflect)]
pub struct Player {
    pub race: Race,
}

/// Component that is attached to the `Player` entity that is currently utilizing local control
/// input.
#[derive(Component, Default)]
pub struct ControlledPlayer {}
