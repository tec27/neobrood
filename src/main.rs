use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use iyes_loopless::prelude::*;
use std::time::Duration;

mod camera;
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

fn main() {
    App::new()
        // TODO(tec27): Use a smaller set of plugins, we really don't need most of this
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "neobrood".into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_fixed_timestep(GameSpeed::Fastest.to_turn_duration(), "fixed_update")
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(camera::CameraControlPlugin)
        .add_plugin(selection::DragSelectionPlugin)
        .add_startup_system(setup)
        .add_system(update_fps_text)
        .run();
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(40.0, 40.0, 0.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.8),
            ..default()
        },
        ..default()
    });

    let font = asset_server.load("fonts/JetbrainsMono-Regular.ttf");
    commands.spawn((
        TextBundle::from_section(
            "FPS: 0",
            TextStyle {
                font: font.clone(),
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
