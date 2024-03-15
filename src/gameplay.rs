use bevy::prelude::*;
use std::time::Duration;

use crate::{
    ecs::despawn_all,
    gamedata::{BwGameData, LoadingAnim},
    players::{ControlledPlayer, Player},
    races::Race,
    states::AppState,
    units::{hq_building, OwnedUnit, UnitType},
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

/// Resource that stores the entity corresponding to a particular player number.
#[derive(Resource, Debug, Default)]
pub struct PlayerEntities {
    entities: Vec<Option<Entity>>,
}

impl PlayerEntities {
    pub fn get(&self, player: u8) -> Option<Entity> {
        self.entities.get(player as usize).copied().flatten()
    }

    pub fn set(&mut self, player: u8, entity: Entity) {
        if player as usize >= self.entities.len() {
            self.entities.resize(player as usize + 1, None);
        }
        self.entities[player as usize] = Some(entity);
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }
}

/// Marker component for entities that should be despawned when exiting the `AppState::InGame`
/// state.
#[derive(Component)]
pub struct InGameOnly;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMode>()
            .init_resource::<PlayerEntities>()
            .add_systems(OnEnter(AppState::InGame), (init_players, init_game).chain())
            .add_systems(OnExit(AppState::InGame), despawn_all::<InGameOnly>);
    }
}

fn init_players(mut commands: Commands, mut player_entities: ResMut<PlayerEntities>) {
    // TODO(tec27): Init this based on lobby structure or whatever
    player_entities.clear();
    let player_id = commands
        .spawn((
            Player { race: Race::Terran },
            ControlledPlayer {},
            InGameOnly,
        ))
        .id();
    player_entities.set(0, player_id);
    let enemy_id = commands
        .spawn((
            Player {
                race: Race::Protoss,
            },
            InGameOnly,
        ))
        .id();
    player_entities.set(1, enemy_id);
}

fn init_game(
    mut commands: Commands,
    mut units: Query<(Entity, &mut UnitType, &OwnedUnit)>,
    player_entities: Res<PlayerEntities>,
    player_query: Query<&Player>,
    game_mode: Res<GameMode>,
    game_data: Res<BwGameData>,
) {
    match *game_mode {
        GameMode::Melee => init_melee_game(
            &mut commands,
            &mut units,
            &player_entities,
            &player_query,
            &game_data,
        ),
        GameMode::MapView => {}
    }
}

fn init_melee_game(
    commands: &mut Commands,
    units: &mut Query<(Entity, &mut UnitType, &OwnedUnit)>,
    player_entities: &Res<PlayerEntities>,
    player_query: &Query<&Player>,
    game_data: &BwGameData,
) {
    for (entity, mut unit_type, owner) in units
        .iter_mut()
        .filter(|(_, u, _)| **u == UnitType::StartLocation)
    {
        let Some(player_entity) = player_entities.get(owner.0) else {
            commands.entity(entity).despawn_recursive();
            continue;
        };
        let Ok(player) = player_query.get(player_entity) else {
            error!(
                "Couldn't find player specified by PlayerEntities: {:?}",
                player_entity
            );
            commands.entity(entity).despawn_recursive();
            continue;
        };

        // TODO(tec27): Use actual player's race, only do this for start locations that are owned by
        // an active player
        let building = hq_building(player.race);
        *unit_type = building;

        let flingy_id = game_data
            .units
            .flingy
            .get(u16::from(building) as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Unit {:?} which isn't a valid ID, using placeholder flingy",
                    building
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
