// NOTE(tec27): These lints are way too sensitive for typical bevy usage, and I think are easy
// enough to catch in reviews anyway
#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use std::path::PathBuf;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};

use gameplay::GameMode;
use gameplay::GameSpeed;
use maps::{load_map, CurrentMap};
use random::LcgRand;
use settings::GameSettings;
use states::AppState;

use crate::fonts::FONT_MONO;

pub mod camera;
pub mod ecs;
pub mod fonts;
pub mod gamedata;
pub mod gameplay;
pub mod main_menu;
pub mod maps;
pub mod math;
pub mod races;
pub mod random;
pub mod render;
pub mod settings;
pub mod states;

pub fn create_app(settings: GameSettings, maps: Vec<PathBuf>) -> App {
    let has_map_args = !maps.is_empty();

    let mut app = App::new();
    // TODO(tec27): Use a smaller set of plugins, we really don't need most of this
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "neobrood".into(),
            present_mode: PresentMode::AutoNoVsync,
            mode: settings.window_mode.into(),
            resolution: WindowResolution::new(
                settings.window_size.map(|(w, _)| w).unwrap_or(1280) as f32,
                settings.window_size.map(|(_, h)| h).unwrap_or(960) as f32,
            ),
            // TODO(tec27): Save and restore position
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }))
    .register_type::<GameSettings>()
    .insert_resource(settings)
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(LoadableMaps { maps, cur_index: 0 })
    .insert_resource(Time::<Fixed>::from_duration(
        GameSpeed::Fastest.to_turn_duration(),
    ))
    .insert_resource(LcgRand::new(0))
    .add_plugins((
        FrameTimeDiagnosticsPlugin,
        camera::CameraControlPlugin,
        gamedata::GameDataPlugin,
        gameplay::GameplayPlugin,
        main_menu::MainMenuPlugin,
        maps::MapsPlugin,
        render::RenderPlugin,
        states::StatesPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, update_fps_text)
    .add_systems(Update, map_navigator.run_if(in_state(AppState::InGame)))
    .add_systems(
        Update,
        bevy::window::close_on_esc.run_if(in_state(AppState::Menu)),
    );

    if has_map_args {
        app.insert_state(AppState::PreGame);
    } else {
        app.insert_state(AppState::Menu);
    }

    #[cfg(feature = "framepacing")]
    app.add_plugins(bevy_framepace::FramepacePlugin);

    #[cfg(feature = "inspector")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app
}

#[derive(Resource, Clone, Debug, Default)]
struct LoadableMaps {
    maps: Vec<PathBuf>,
    cur_index: usize,
}

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_map: ResMut<CurrentMap>,
    mut next_state: ResMut<NextState<AppState>>,
    settings: Res<GameSettings>,
    loadable_maps: Res<LoadableMaps>,
) {
    info!("Using settings: {:?}", *settings);

    if !loadable_maps.maps.is_empty() {
        commands.insert_resource(GameMode::MapView);
        let map_path = loadable_maps.maps.first().cloned().unwrap();
        load_map(
            &map_path,
            &mut current_map,
            &mut next_state,
            &asset_server,
            &settings,
        );
    } else {
        commands.insert_resource(GameMode::Melee);
    }

    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load(FONT_MONO);
    commands.spawn((
        TextBundle::from_section(
            "FPS: 0",
            TextStyle {
                font,
                font_size: 16.0,
                color: Color::rgb(0.7, 0.7, 0.7),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(2.0),
            top: Val::Px(2.0),
            ..default()
        }),
        FpsText,
    ));
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
            fps = fps_smoothed;
        }
    }

    let mut text = query.single_mut();
    text.sections[0].value = format!("FPS: {:.1}", fps);
}

fn map_navigator(
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut current_map: ResMut<CurrentMap>,
    mut loadable_maps: ResMut<LoadableMaps>,
    settings: Res<GameSettings>,
) {
    if keys.just_pressed(KeyCode::Space) && loadable_maps.maps.len() > 1 {
        loadable_maps.cur_index = (loadable_maps.cur_index + 1) % loadable_maps.maps.len();
        let map_path = loadable_maps.maps[loadable_maps.cur_index].clone();
        load_map(
            &map_path,
            &mut current_map,
            &mut next_state,
            &asset_server,
            &settings,
        );
    }
}
