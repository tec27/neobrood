use bevy::prelude::*;

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

/// Resource that stores the entity corresponding to a particular player number.
#[derive(Resource, Debug, Default)]
pub struct PlayerEntities {
    entities: Vec<Option<Entity>>,
}

impl PlayerEntities {
    /// Retrieve the player entity that corresponds to a given player number (if any).
    pub fn get(&self, player: u8) -> Option<Entity> {
        self.entities.get(player as usize).copied().flatten()
    }

    /// Sets the player entity that corresponds to a given player number.
    pub fn set(&mut self, player: u8, entity: Entity) {
        if player as usize >= self.entities.len() {
            self.entities.resize(player as usize + 1, None);
        }
        self.entities[player as usize] = Some(entity);
    }

    /// Clears all registered player entities.
    pub fn clear(&mut self) {
        self.entities.clear();
    }

    /// Returns the player number for the given entity, if it has one.
    pub fn player_num_for(&self, entity: Entity) -> Option<u8> {
        self.entities
            .iter()
            .position(|&e| e == Some(entity))
            .map(|i| i as u8)
    }
}
