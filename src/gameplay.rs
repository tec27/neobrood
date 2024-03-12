use bevy::prelude::*;
use std::time::Duration;

use crate::{
    gamedata::{BwGameData, LoadingAnim},
    races::Race,
    states::AppState,
    units::{hq_building, UnitType, START_LOCATION},
};

#[allow(dead_code)]
pub enum GameSpeed {
    Slowest,
    Slower,
    Slow,
    Normal,
    Fast,
    Faster,
    Fastest,
}

impl GameSpeed {
    pub fn to_turn_duration(&self) -> Duration {
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

/// What type of game is being played.
#[derive(Resource, Debug, Default)]
pub enum GameMode {
    /// A game with no teams and standard "kill all buildings" objectives. Alliance changes are
    /// allowed.
    #[default]
    Melee,
    /// View the map with no objectives/interaction.
    MapView,
    // TODO(tec27): Implement more game modes
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMode>()
            .add_systems(OnEnter(AppState::InGame), init_game);
    }
}

fn init_game(
    mut commands: Commands,
    mut units: Query<(Entity, &mut UnitType)>,
    game_mode: Res<GameMode>,
    game_data: Res<BwGameData>,
) {
    match *game_mode {
        GameMode::Melee => init_melee_game(&mut commands, &mut units, &game_data),
        GameMode::MapView => {}
    }
}

fn init_melee_game(
    commands: &mut Commands,
    units: &mut Query<(Entity, &mut UnitType)>,
    game_data: &BwGameData,
) {
    for (entity, mut unit_type) in units.iter_mut().filter(|(_, u)| **u == START_LOCATION) {
        // TODO(tec27): Use actual player's race, only do this for start locations that are owned by
        // an active player
        let building = hq_building(Race::Terran);
        *unit_type = building;

        let flingy_id = game_data
            .units
            .flingy
            .get(building.0 as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Unit {} which isn't a valid ID, using placeholder flingy",
                    building.0
                );
                0
            });
        let sprite_id = game_data
            .flingy
            .sprite
            .get(flingy_id as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Flingy {} which isn't a valid ID, using placeholder sprite",
                    flingy_id
                );
                0
            });
        let image_id = game_data
            .sprites
            .image
            .get(sprite_id as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Sprite {} which isn't a valid ID, using placeholder sprite",
                    sprite_id
                );
                0
            });

        // TODO(tec27): Maybe we should have a change handler for UnitType that does this instead?
        // Could also use that for initializing the placed unit's in the first place
        commands
            .entity(entity)
            .despawn_descendants()
            .with_children(|builder| {
                builder.spawn(LoadingAnim::new(image_id));
            });
    }
}
