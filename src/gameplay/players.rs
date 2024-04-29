use bevy::prelude::*;

use crate::races::Race;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .register_type::<ControlledPlayer>()
        .register_type::<PlayerNumber>()
        .init_resource::<PlayerEntities>()
        .add_systems(
            // TODO(tec27): May need to run this at a different time? Or not do this via a system
            // and instead structure PlayerEntities differently to manage this?
            PreUpdate,
            update_player_numbers.run_if(resource_changed::<PlayerEntities>),
        );
}

/// Component that specifies a player in the game. This is used for both human and AI players.
#[allow(unused)]
#[derive(Component, Debug, Default, Reflect)]
pub struct Player {
    pub race: Race,
}

/// Component that is attached to the `Player` entity that is currently utilizing local control
/// input.
#[derive(Component, Default, Reflect)]
pub struct ControlledPlayer;

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

/// A component that specifies the player number for a given [Player] entity.
#[derive(Component, Debug, Copy, Clone, Default, Reflect)]
pub struct PlayerNumber(pub u8);

fn update_player_numbers(
    player_entities: ResMut<PlayerEntities>,
    mut query: Query<(Entity, Option<&mut PlayerNumber>), With<Player>>,
    mut commands: Commands,
) {
    for (e, player_number) in query.iter_mut() {
        let new_number = player_entities.player_num_for(e);
        match player_number {
            Some(mut p) => {
                if let Some(n) = new_number {
                    if p.0 != n {
                        p.0 = n;
                    }
                } else {
                    commands.entity(e).remove::<PlayerNumber>();
                }
            }
            None => {
                if let Some(n) = new_number {
                    commands.entity(e).insert(PlayerNumber(n));
                }
            }
        }
    }
}
