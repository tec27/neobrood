use crate::maps::CurrentMap;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_ecs_tilemap::prelude::TileStorage;
use directories::UserDirs;
use iyes_loopless::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

mod camera;
mod maps;
mod selection;

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
    let settings = match File::open(settings_file) {
        Ok(mut file) => serde_json::from_reader::<_, GameSettings>(&mut file).unwrap_or_else(|e| {
            warn!(
                "Using default settings due to error parsing settings file: {}",
                e
            );
            GameSettings::default()
        }),
        Err(e) => {
            warn!(
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

    App::new()
        // TODO(tec27): Use a smaller set of plugins, we really don't need most of this
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "neobrood".into(),
                present_mode: PresentMode::AutoNoVsync,
                mode: settings.window_mode.into(),
                width: settings.window_size.map(|(w, _)| w).unwrap_or(1280) as f32,
                height: settings.window_size.map(|(_, h)| h).unwrap_or(960) as f32,
                position: WindowPosition::Centered,
                ..default()
            },
            ..default()
        }))
        .insert_resource(settings)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(LoadableMaps { maps, cur_index: 0 })
        .add_fixed_timestep(GameSpeed::Fastest.to_turn_duration(), "fixed_update")
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy_framepace::FramepacePlugin)
        // .add_plugin(WorldInspectorPlugin)
        .add_plugin(camera::CameraControlPlugin)
        .add_plugin(maps::MapsPlugin)
        .add_plugin(selection::DragSelectionPlugin)
        .add_startup_system(setup)
        .add_system(update_fps_text)
        .add_system(map_navigator)
        .add_system(map_drag_and_drop)
        // TODO(tec27): Remove this once we have actual game stuff
        .add_system(bevy::window::close_on_esc)
        .run();
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

    let map_path = loadable_maps
        .maps
        .get(0)
        .cloned()
        .unwrap_or(PathBuf::from("lt.scm"));
    current_map.handle = asset_server.load(map_path);

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
            position: UiRect {
                left: Val::Px(2.0),
                top: Val::Px(2.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));
}

fn update_fps_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
            fps = fps_smoothed;
        }
    }

    let mut text = query.single_mut();
    text.sections[0].value = format!("FPS: {:.1}", fps);
}

fn map_navigator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    tilemaps: Query<Entity, With<TileStorage>>,
    mut current_map: ResMut<CurrentMap>,
    mut loadable_maps: ResMut<LoadableMaps>,
) {
    if keys.just_pressed(KeyCode::Space) && loadable_maps.maps.len() > 1 {
        for entity in tilemaps.iter() {
            commands.entity(entity).despawn_recursive();
        }

        loadable_maps.cur_index = (loadable_maps.cur_index + 1) % loadable_maps.maps.len();
        let map_path = loadable_maps.maps[loadable_maps.cur_index].clone();
        info!("Loading map: {}", map_path.to_string_lossy());
        current_map.handle = asset_server.load(map_path);
    }
}

fn map_drag_and_drop(
    mut drop_events: EventReader<FileDragAndDrop>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemaps: Query<Entity, With<TileStorage>>,
    mut current_map: ResMut<CurrentMap>,
) {
    for event in drop_events.iter() {
        let FileDragAndDrop::DroppedFile { path_buf, .. } = event else {
            continue;
        };

        let extension = path_buf.extension().map_or("".into(), |s| {
            s.to_ascii_lowercase().to_string_lossy().to_string()
        });
        if extension == "scm" || extension == "scx" {
            for entity in tilemaps.iter() {
                commands.entity(entity).despawn_recursive();
            }

            info!("Loading map: {}", path_buf.to_string_lossy());
            current_map.handle = asset_server.load(path_buf.clone());
        }
    }
}
