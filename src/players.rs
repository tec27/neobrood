use bevy::ecs::component::Component;

use crate::races::Race;

/// Component that specifies a player in the game. This is used for both human and AI players.
#[allow(unused)]
#[derive(Component, Debug, Default)]
struct Player {
    race: Race,
}

/// Component that is attached to the `Player` entity that is currently utilizing local control
/// input.
#[derive(Component, Default)]
struct ControlledPlayer {}
