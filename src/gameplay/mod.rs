use bevy::{prelude::*, utils::hashbrown::HashMap};
use broodmap::chk::sprites::SpriteFlags;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    gamedata::{BwGameData, ConstructTypeId, CONSTRUCTS, SPRITES},
    gameplay::constructs::{ConstructImageBundle, ConstructSpriteBundle},
    maps::{
        game_map::GameMap,
        position::{self, Position},
        CurrentMap, MapAsset,
    },
    races::Race,
    random::LockedLcgRand,
    render::ysort::YSort,
    states::{AppState, InGameOnly},
};

use self::{
    create_construct::{CreateConstructEvent, CreationKind},
    facing_direction::apply_facing_to_images,
    gizmos::{show_construct_gizmos, ConstructGizmos},
    iscripts::exec_iscripts,
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
pub mod iscripts;
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
            .add_plugins(constructs::plugin)
            .add_plugins(players::plugin)
            .register_type::<ConstructGizmos>()
            .init_resource::<GameMode>()
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
            .add_systems(
                FixedUpdate,
                exec_iscripts.run_if(in_state(AppState::InGame)),
            )
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
    player_entities: Res<PlayerEntities>,
    player_query: Query<&Player>,
    game_mode: Res<GameMode>,
    mut creation_events: EventWriter<CreateConstructEvent>,
    current_map: Res<CurrentMap>,
    map_assets: Res<Assets<MapAsset>>,
) {
    let map = map_assets
        .get(&current_map.handle)
        .expect("Current map not loaded");

    let is_ums = match *game_mode {
        GameMode::Melee => false,
        GameMode::MapView => true,
    };

    create_map_sprites(is_ums, &mut commands, map, &mut creation_events);
    create_placed_units(is_ums, map, &mut creation_events, &player_entities);

    let mut start_locations = HashMap::new();
    for unit in map
        .placed_units
        .iter()
        .filter(|u| ConstructTypeId::from(u.unit_id) == ConstructTypeId::StartLocation)
    {
        let Some(owner) = unit.owner else {
            continue;
        };
        if owner < 8 {
            start_locations.insert(owner, Position::new(unit.x as i32, unit.y as i32));
        }
    }

    match *game_mode {
        GameMode::Melee => init_melee_game(
            &player_entities,
            &player_query,
            &mut creation_events,
            &start_locations,
        ),
        GameMode::MapView => init_map_view(&mut commands, &start_locations),
    }
}

fn create_map_sprites(
    is_ums: bool,
    commands: &mut Commands,
    map: &MapAsset,
    creation_events: &mut EventWriter<CreateConstructEvent>,
) {
    info!(
        "Creating map sprites, map has {} sprites",
        map.sprites.len()
    );

    for (i, sprite) in map.sprites.iter().enumerate() {
        if sprite.flags.contains(SpriteFlags::DRAW_AS_SPRITE) {
            // This is a pure sprite, rather than a unit sprite
            // See http://www.staredit.net/wiki/index.php?title=Sprite

            let Some(s) = SPRITES.get(sprite.id as usize) else {
                warn!(
                    "Encountered Sprite {} which isn't a valid ID, skipping",
                    sprite.id
                );
                continue;
            };

            // TODO(tec27): I don't think this implementation is entirely correct, need to delve
            // more into what a "Thingy" entails for our components
            // TODO(tec27): I'm pretty sure these get iscripts as well, restructure code to make
            // that easier to initialize without duplicating the code from create_construct
            commands
                .spawn((
                    SpatialBundle::default(),
                    Position::new(sprite.x.into(), sprite.y.into()),
                    YSort(2.0),
                    Name::new(format!("PureSprite #{i}")),
                    InGameOnly,
                ))
                .with_children(|builder| {
                    builder
                        .spawn(ConstructSpriteBundle::new(s.id))
                        .with_children(|builder| {
                            builder.spawn(ConstructImageBundle::new(s.image_id));
                        });
                });
        } else {
            // This is a unit sprite
            // See http://www.staredit.net/wiki/index.php?title=Unit#Unit_sprites
            let construct_type: ConstructTypeId = sprite.id.into();
            if let ConstructTypeId::Unknown(c) = construct_type {
                warn!("Encountered Unit Sprite {c} which isn't a valid ID, skipping",);
                continue;
            };

            let owner = match construct_type {
                ConstructTypeId::SpecialUpperLevelDoor
                | ConstructTypeId::SpecialRightUpperLevelDoor
                | ConstructTypeId::SpecialPitDoor
                | ConstructTypeId::SpecialRightPitDoor => 11,
                _ => sprite.owner,
            };

            if is_ums || owner == 11 {
                // TODO(tec27): These have different HP initialization logic than normal constructs,
                // we need to deal with that (see wiki entry above)
                creation_events.send(CreateConstructEvent {
                    construct_type,
                    position: Some(Position::new(sprite.x as i32, sprite.y as i32)),
                    owner: Some(owner),
                    kind: CreationKind::Immediate,
                });
                // TODO(tec27): Deal with disabled unit sprites
            }
        }
    }
}

fn create_placed_units(
    is_ums: bool,
    map: &MapAsset,
    creation_events: &mut EventWriter<CreateConstructEvent>,
    player_entities: &Res<PlayerEntities>,
) {
    info!(
        "Creating placed units, map has {} placed units",
        map.placed_units.len()
    );

    for unit in map.placed_units.iter() {
        if CONSTRUCTS.get(unit.unit_id as usize).is_none() {
            // TODO(tec27): If the unit ID is 0xffff it seems like we need to poke the RNG once

            warn!(
                "Encountered construct {} which isn't a valid ID, skipping",
                unit.unit_id
            );
            continue;
        };

        let construct_type: ConstructTypeId = unit.unit_id.into();

        if construct_type == ConstructTypeId::StartLocation {
            // Start locations are handled separately
            continue;
        }

        // TODO(tec27): The logic for creating units comes from OpenBW, but doesn't seem to actually
        // match SC:R's behavior (owner 12, non-hallucinated units don't seem to be created)
        if let Some(owner) = unit.owner {
            if owner >= 12 {
                warn!(
                    "Found construct with invalid owner {:?}, skipping",
                    unit.owner
                );
                continue;
            }

            // TODO(tec27): Also deal with the weird slot types, e.g. rescue
            if owner < 8 && player_entities.get(owner).is_none() {
                // Skip units for any players that are not present
                continue;
            }
        }

        let has_neutral_owner = unit.owner.map_or(true, |o| o == 11);
        if is_ums || has_neutral_owner || construct_type.is_neutral() {
            // TODO(tec27): UMS has some special additional logic around creating units for certain
            // players/forces that needs to be handled

            // TODO(tec27): Pass all the health/shield/etc. values through
            creation_events.send(CreateConstructEvent {
                construct_type,
                position: Some(Position::new(unit.x as i32, unit.y as i32)),
                owner: unit.owner,
                kind: CreationKind::Immediate,
            });
        }
    }
}

fn init_melee_game(
    player_entities: &Res<PlayerEntities>,
    player_query: &Query<&Player>,
    creation_events: &mut EventWriter<CreateConstructEvent>,
    start_locations: &HashMap<u8, Position>,
) {
    for i in 0..8 {
        let Some(player_entity) = player_entities.get(i) else {
            continue;
        };
        let Ok(player) = player_query.get(player_entity) else {
            error!(
                "Couldn't find player specified by PlayerEntities: {:?}",
                player_entity
            );
            continue;
        };
        let Some(position) = start_locations.get(&i) else {
            error!("Player {i:?} doesn't have a start location");
            continue;
        };

        // TODO(tec27): Need to also destroy any constructs that are within the bounds of the HQ
        // building

        let building = player.race.hq_building();
        creation_events.send(CreateConstructEvent {
            construct_type: building,
            position: Some(*position),
            owner: Some(i),
            kind: CreationKind::Immediate,
        });

        let worker = player.race.worker();
        creation_events.send_batch((0..4).map(|_| CreateConstructEvent {
            construct_type: worker,
            position: Some(*position),
            owner: Some(i),
            kind: CreationKind::Immediate,
        }));
    }
}

fn init_map_view(commands: &mut Commands<'_, '_>, start_locations: &HashMap<u8, Position>) {
    let sprite_id = ConstructTypeId::StartLocation.flingy().sprite_id;
    let image_id = ConstructTypeId::StartLocation.flingy().sprite().image_id;

    for (player, position) in start_locations {
        commands
            .spawn((
                SpatialBundle::default(),
                *position,
                YSort(2.0),
                Name::new(format!("Start Location {player}")),
                InGameOnly,
            ))
            .with_children(|builder| {
                builder
                    .spawn(ConstructSpriteBundle::new(sprite_id))
                    .with_children(|builder| {
                        builder.spawn(ConstructImageBundle::new(image_id));
                    });
            });
    }
}
