use bevy::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    constructs::OwnedConstruct,
    gamedata::{BwGameData, ConstructTypeId, LoadingAnim},
    maps::{
        game_map::GameMap,
        position::{self, Position},
    },
    players::{ControlledPlayer, Player},
    races::Race,
    random::LcgRand,
    states::{AppState, InGameOnly},
};

use self::{
    create_construct::CreateConstructExt,
    gizmos::{show_construct_gizmos, ConstructGizmos},
};

pub mod create_construct;
pub mod gizmos;

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
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ConstructGizmos>()
            .init_resource::<GameMode>()
            .init_resource::<PlayerEntities>()
            .init_gizmo_group::<ConstructGizmos>()
            .add_systems(OnEnter(AppState::PreGame), init_random)
            .add_systems(Update, proceed_to_game.run_if(in_state(AppState::PreGame)))
            .add_systems(OnEnter(AppState::InGame), (init_players, init_game).chain())
            .add_systems(
                PostUpdate,
                (show_construct_gizmos)
                    .after(position::position_to_transform)
                    .run_if(|store: Res<GizmoConfigStore>| {
                        store.config::<ConstructGizmos>().0.enabled
                    }),
            );
    }
}

fn init_random(mut lcg: ResMut<LcgRand>) {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock set incorrectly")
        .as_millis() as u32;
    lcg.reseed(seed);
    info!("Seeded RNG with {seed}");
}

fn proceed_to_game(
    mut next_state: ResMut<NextState<AppState>>,
    game_data: Option<Res<BwGameData>>,
    game_map: Query<Entity, With<GameMap>>,
) {
    if game_data.is_some() && !game_map.is_empty() {
        next_state.set(AppState::InGame);
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
    player_entities.set(1, player_id);
    let enemy_id = commands
        .spawn((
            Player {
                race: Race::Protoss,
            },
            InGameOnly,
        ))
        .id();
    player_entities.set(0, enemy_id);
}

fn init_game(
    mut commands: Commands,
    mut units: Query<(Entity, &mut ConstructTypeId, &OwnedConstruct, &Position)>,
    player_entities: Res<PlayerEntities>,
    player_query: Query<&Player>,
    game_mode: Res<GameMode>,
) {
    match *game_mode {
        GameMode::Melee => {
            init_melee_game(&mut commands, &mut units, &player_entities, &player_query)
        }
        GameMode::MapView => {}
    }
}

fn init_melee_game(
    commands: &mut Commands,
    units: &mut Query<(Entity, &mut ConstructTypeId, &OwnedConstruct, &Position)>,
    player_entities: &Res<PlayerEntities>,
    player_query: &Query<&Player>,
) {
    for (entity, mut construct_type, owner, position) in units
        .iter_mut()
        .filter(|(_, u, _, _)| **u == ConstructTypeId::StartLocation)
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

        let building = if player.race != Race::Terran {
            player.race.hq_building()
        } else {
            ConstructTypeId::TerranBarracks.def()
        };
        *construct_type = building.type_id();

        let image_id = building.flingy().sprite().image_id;

        // TODO(tec27): Need to also destroy any constructs that are within the bounds of the HQ
        // building

        // TODO(tec27): Maybe we should have a change handler for UnitType that does this instead?
        // Could also use that for initializing the placed unit's in the first place
        commands
            .entity(entity)
            .despawn_descendants()
            .with_children(|builder| {
                builder.spawn(LoadingAnim::new(image_id));
            });

        warn!("{:?} is at {:?}", *construct_type, position);

        let worker_type = if player.race != Race::Terran {
            player.race.worker()
        } else {
            ConstructTypeId::TerranMarine.def()
        };
        for _ in 0..100 {
            commands.create_and_place_construct(worker_type.type_id(), *position, Some(owner.0))
        }
    }
}
