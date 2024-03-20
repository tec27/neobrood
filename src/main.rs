// NOTE(tec27): This lint is way too sensitive for typical bevy queries, and I think is easy enough
// to catch in reviews anyway
#![allow(clippy::type_complexity)]

use std::env;
use std::fs::File;
use std::path::PathBuf;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};
use directories::UserDirs;
use neobrood::asset_packs::{AssetPack, AssetQuality};
use neobrood::gameplay::GameMode;
use neobrood::gameplay::GameSpeed;
use neobrood::maps::{CurrentMap, MapAssetSettings};
use neobrood::random::LcgRand;
use neobrood::states::AppState;
use serde::{Deserialize, Serialize};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
enum NeobroodWindowMode {
    Windowed,
    #[default]
    BorderlessFullscreen,
    ExclusiveFullscreen,
}

impl From<NeobroodWindowMode> for WindowMode {
    fn from(value: NeobroodWindowMode) -> Self {
        match value {
            NeobroodWindowMode::Windowed => WindowMode::Windowed,
            NeobroodWindowMode::BorderlessFullscreen => WindowMode::BorderlessFullscreen,
            NeobroodWindowMode::ExclusiveFullscreen => WindowMode::Fullscreen,
        }
    }
}

// TODO(tec27): Write a way to configure these ingame and save them to the file
#[derive(Resource, Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameSettings {
    #[serde(default)]
    window_mode: NeobroodWindowMode,
    window_size: Option<(u32, u32)>,
}

#[derive(Resource, Clone, Debug, Default)]
struct LoadableMaps {
    maps: Vec<PathBuf>,
    cur_index: usize,
}

fn main() {
    let user_dirs = UserDirs::new().expect("Couldn't get user directories!");
    let documents_dir = user_dirs
        .document_dir()
        .expect("Couldn't get Documents directory!");
    let settings_file = documents_dir
        .join("Starcraft")
        .join("neobrood-settings.json");
    // NOTE(tec27): We avoid using any tracing functions for logging here as that won't be
    // initialized until Bevy's LogPlugin is
    let settings = match File::open(settings_file) {
        Ok(mut file) => serde_json::from_reader::<_, GameSettings>(&mut file).unwrap_or_else(|e| {
            eprintln!(
                "Using default settings due to error parsing settings file: {}",
                e
            );
            GameSettings::default()
        }),
        Err(e) => {
            eprintln!(
                "Using default settings due to error reading settings file: {}",
                e
            );
            GameSettings::default()
        }
    };

    let maps = env::args()
        .skip(1)
        .flat_map(|path| {
            let mut path = PathBuf::from(path);
            if !path.is_absolute() {
                // Bevy will treat relative paths as relative to `assets/`, so we "fix" that here so
                // any relative paths are relative to the program dir
                path = env::current_dir().unwrap_or(PathBuf::from("..")).join(path);
            }

            if path.is_dir() {
                path.read_dir()
                    .expect("Couldn't read specified map directory")
                    .filter_map(|entry| {
                        let entry = entry.expect("Couldn't read directory entry");
                        let path = entry.path();
                        let extension = path.extension().map_or("".into(), |s| {
                            s.to_ascii_lowercase().to_string_lossy().to_string()
                        });
                        if extension == "scm" || extension == "scx" {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                vec![path]
            }
        })
        .collect::<Vec<_>>();

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
    .insert_resource(settings)
    // TODO(tec27): Pull from settings
    .insert_resource(AssetQuality::High)
    // TODO(tec27): Pull from settings
    .insert_resource(AssetPack::Standard)
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(LoadableMaps { maps, cur_index: 0 })
    .insert_resource(Time::<Fixed>::from_duration(
        GameSpeed::Fastest.to_turn_duration(),
    ))
    .insert_resource(LcgRand::new(0))
    .add_plugins((
        FrameTimeDiagnosticsPlugin,
        neobrood::camera::CameraControlPlugin,
        neobrood::gamedata::GameDataPlugin,
        neobrood::gameplay::GameplayPlugin,
        neobrood::main_menu::MainMenuPlugin,
        neobrood::maps::MapsPlugin,
        neobrood::render::RenderPlugin,
        neobrood::selection::DragSelectionPlugin,
        neobrood::states::StatesPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, update_fps_text)
    .add_systems(Update, map_navigator.run_if(in_state(AppState::InGame)))
    // TODO(tec27): Remove this once we have actual game stuff
    .add_systems(Update, bevy::window::close_on_esc);

    if has_map_args {
        app.insert_state(AppState::PreGame);
    } else {
        app.insert_state(AppState::Menu);
    }

    #[cfg(feature = "framepacing")]
    app.add_plugins(bevy_framepace::FramepacePlugin);

    #[cfg(feature = "inspector")]
    app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_map: ResMut<CurrentMap>,
    settings: Res<GameSettings>,
    loadable_maps: Res<LoadableMaps>,
) {
    info!("Using settings: {:?}", *settings);

    if !loadable_maps.maps.is_empty() {
        commands.insert_resource(GameMode::MapView);
        let map_path = loadable_maps.maps.first().cloned().unwrap();
        current_map.handle = asset_server.load(map_path);
    } else {
        commands.insert_resource(GameMode::Melee);
    }

    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/JetbrainsMono-Regular.ttf");
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
    asset_quality: Res<AssetQuality>,
    asset_pack: Res<AssetPack>,
) {
    if keys.just_pressed(KeyCode::Space) && loadable_maps.maps.len() > 1 {
        next_state.set(AppState::PreGame);

        loadable_maps.cur_index = (loadable_maps.cur_index + 1) % loadable_maps.maps.len();
        let map_path = loadable_maps.maps[loadable_maps.cur_index].clone();
        // TODO(tec27): Pull out a utility to do this loading logic/update current_map
        info!("Loading map: {}", map_path.to_string_lossy());
        let asset_quality = *asset_quality;
        let asset_pack = *asset_pack;
        current_map.handle =
            asset_server.load_with_settings(map_path, move |settings: &mut MapAssetSettings| {
                settings.quality = asset_quality;
                settings.pack = asset_pack;
            });
    }
}
