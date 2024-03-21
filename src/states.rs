use bevy::prelude::*;

use crate::ecs::despawn_all;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    /// State that shows a menu for selecting options and starting a game.
    #[default]
    Menu,
    /// Transitory state that ensures the necessary game resources are loaded before a game starts.
    /// This will transition to `InGame` once the resources are ready.
    PreGame,
    /// State that runs actual gameplay.
    InGame,
}

/// Marker component for entities that should be despawned when exiting the `AppState::InGame`
/// state.
#[derive(Component, Copy, Clone, Reflect)]
pub struct InGameOnly;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InGameOnly>()
            .add_systems(Update, log_transitions)
            .add_systems(OnExit(AppState::InGame), despawn_all::<InGameOnly>);
    }
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<AppState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.before, transition.after
        );
    }
}
