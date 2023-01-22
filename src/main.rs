use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::time::Duration;

#[allow(dead_code)]
enum GameSpeed {
    Slowest,
    Slower,
    Slow,
    Normal,
    Fast,
    Faster,
    Fastest,
}

impl GameSpeed {
    fn to_turn_duration(&self) -> Duration {
        match self {
            GameSpeed::Slowest => Duration::from_millis(167),
            GameSpeed::Slower => Duration::from_millis(111),
            GameSpeed::Slow => Duration::from_millis(83),
            GameSpeed::Normal => Duration::from_millis(67),
            GameSpeed::Fast => Duration::from_millis(56),
            GameSpeed::Faster => Duration::from_millis(48),
            GameSpeed::Fastest => Duration::from_millis(42),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_fixed_timestep(GameSpeed::Fastest.to_turn_duration(), "fixed_update")
        .run();
}
