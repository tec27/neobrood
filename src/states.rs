use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::EventReader,
        schedule::{StateTransitionEvent, States},
    },
    log::info,
};

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

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_transitions);
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
