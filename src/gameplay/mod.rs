use bevy::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    gamedata::{BwGameData, ConstructTypeId},
    maps::{
        game_map::GameMap,
        position::{self, Position},
    },
    races::Race,
    random::LockedLcgRand,
    states::{AppState, InGameOnly},
};

use self::{
    constructs::{ConstructImage, ConstructSprite, OwnedConstruct},
    create_construct::{CreateConstructEvent, CreationKind},
    facing_direction::apply_facing_to_images,
    gizmos::{show_construct_gizmos, ConstructGizmos},
    players::{ControlledPlayer, Player, PlayerEntities},
    selection::SelectedEntities,
};

pub mod build_time;
pub mod constructs;
pub mod create_construct;
pub mod facing_direction;
pub mod gizmos;
pub mod health;
mod in_game_menu;
pub mod players;
pub mod selection;
pub mod shield;
pub mod sounds;
pub mod status;

pub use in_game_menu::InGameMenuState;

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
        app.add_plugins(in_game_menu::InGameMenuPlugin)
            .add_plugins(selection::DragSelectionPlugin)
            .add_plugins(create_construct::plugin)
            .register_type::<ConstructGizmos>()
            .register_type::<ConstructImage>()
            .register_type::<ConstructSprite>()
            .register_type::<OwnedConstruct>()
            .init_resource::<GameMode>()
            .init_resource::<PlayerEntities>()
            .insert_gizmo_group(
                ConstructGizmos::default(),
                GizmoConfig {
                    enabled: false,
                    ..default()
                },
            )
            .add_systems(OnEnter(AppState::PreGame), init_random)
            .add_systems(Update, proceed_to_game.run_if(in_state(AppState::PreGame)))
            .add_systems(OnEnter(AppState::InGame), (init_players, init_game).chain())
            .add_systems(Update, apply_facing_to_images)
            .add_systems(
                PostUpdate,
                (show_construct_gizmos)
                    .after(position::apply_position_to_transform)
                    .run_if(|store: Res<GizmoConfigStore>| {
                        store.config::<ConstructGizmos>().0.enabled
                    }),
            );
    }
}

fn init_random(mut lcg: ResMut<LockedLcgRand>) {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock set incorrectly")
        .as_millis() as u32;
    lcg.i_know_what_im_doing_please_reseed(seed);
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
            SelectedEntities::default(),
            InGameOnly,
        ))
        .id();
    player_entities.set(1, player_id);
    let enemy_id = commands
        .spawn((
            Player {
                race: Race::Protoss,
            },
            SelectedEntities::default(),
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
    mut creation_events: EventWriter<CreateConstructEvent>,
) {
    match *game_mode {
        GameMode::Melee => init_melee_game(
            &mut commands,
            &mut units,
            &player_entities,
            &player_query,
            &mut creation_events,
        ),
        GameMode::MapView => {}
    }
}

fn init_melee_game(
    commands: &mut Commands,
    units: &mut Query<(Entity, &mut ConstructTypeId, &OwnedConstruct, &Position)>,
    player_entities: &Res<PlayerEntities>,
    player_query: &Query<&Player>,
    creation_events: &mut EventWriter<CreateConstructEvent>,
) {
    for (entity, _c, owner, position) in units
        .iter_mut()
        .filter(|(_, c, _, _)| **c == ConstructTypeId::StartLocation)
    {
        commands.entity(entity).despawn_recursive();
        let Some(player_entity) = player_entities.get(owner.0) else {
            continue;
        };
        let Ok(player) = player_query.get(player_entity) else {
            error!(
                "Couldn't find player specified by PlayerEntities: {:?}",
                player_entity
            );
            continue;
        };

        // TODO(tec27): Need to also destroy any constructs that are within the bounds of the HQ
        // building

        let building = player.race.hq_building();
        creation_events.send(CreateConstructEvent {
            construct_type: building,
            position: Some(*position),
            owner: Some(owner.0),
            kind: CreationKind::Immediate,
        });

        let worker = player.race.worker();
        creation_events.send_batch((0..4).map(|_| CreateConstructEvent {
            construct_type: worker,
            position: Some(*position),
            owner: Some(owner.0),
            kind: CreationKind::Immediate,
        }));
    }
}
