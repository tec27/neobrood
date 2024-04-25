use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};

use crate::gameplay::InGameMenuState;
use crate::maps::game_map::{GameMap, GameMapSize};
use crate::settings::GameSettings;
use crate::states::AppState;

/// How far from the edge of the screen the mouse needs to be to start scrolling, in pixels.
const EDGE_SCROLL_PX: f32 = 4.0;
const MOUSE_PAN_SPEED: f32 = 3000.0;
/// How fast the camera zooms in/out from scrolling.
const MOUSE_ZOOM_SPEED: f32 = 0.5;

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPanLocked>()
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (handle_window_focus, camera_control).run_if(
                    in_state(AppState::InGame).and_then(in_state(InGameMenuState::Disabled)),
                ),
            );
    }
}

/// A resource that indicates whether the camera should be locked to its current position. This
/// typically occurs because a drag selection is in progress.
#[derive(Resource, Default, Debug)]
pub struct CameraPanLocked(pub bool);

fn setup(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    let mut window = window.get_single_mut().unwrap();
    info!("Confining mouse and centering it within the window...");
    let cursor_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    window.focused = true;
    window.set_cursor_position(Some(cursor_pos));
    window.cursor.grab_mode = CursorGrabMode::Confined;

    // Reset the camera to the default position/zoom
    // TODO(tec27): Center on base location if there is one
    let (mut transform, mut projection) = camera_query.single_mut();
    transform.translation = Vec3::splat(0.0);
    projection.scale = 1.0;
}

fn handle_window_focus(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventReader<WindowFocused>,
) {
    for event in events.read() {
        if !window.contains(event.window) {
            continue;
        }

        let mut window = window.get_single_mut().unwrap();
        window.cursor.grab_mode = if event.focused {
            info!("Window gained focus, confining cursor...");
            CursorGrabMode::Confined
        } else {
            info!("Window lost focus, releasing cursor...");
            CursorGrabMode::None
        };
    }
}

#[derive(Default)]
struct CameraControlState {
    last_pos: Vec2,
}

fn camera_control(
    mut state: Local<CameraControlState>,
    time: Res<Time>,
    camera_pan_locked: Res<CameraPanLocked>,
    window: Query<&Window, (With<PrimaryWindow>, Without<Camera>)>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut scroll_events: EventReader<MouseWheel>,
    settings: Res<GameSettings>,
    map_size: Query<&GameMapSize, With<GameMap>>,
) {
    // TODO(tec27): implement arrow key scrolling + middle mouse panning as well
    let window = window.get_single().unwrap();
    let mouse_position = window.cursor_position().unwrap_or(state.last_pos);
    state.last_pos = mouse_position;

    if camera_pan_locked.0 {
        return;
    }

    let scroll_delta = scroll_events.read().fold(0.0, |acc, event| {
        acc - match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / 20.0,
        }
    });
    if scroll_delta != 0.0 {
        let (_, mut projection) = camera_query.single_mut();
        let scale_change = scroll_delta * MOUSE_ZOOM_SPEED;
        let mut log_scale = projection.scale.ln();
        log_scale += scale_change;

        projection.scale = log_scale.exp().clamp(0.25, 10.0);
    }

    let width = window.width();
    let height = window.height();

    let mut pan_x = 0.0;
    let mut pan_y = 0.0;

    if mouse_position.x < EDGE_SCROLL_PX {
        pan_x = -1.0;
    } else if mouse_position.x > width - EDGE_SCROLL_PX {
        pan_x = 1.0;
    }

    if mouse_position.y < EDGE_SCROLL_PX {
        pan_y = 1.0;
    } else if mouse_position.y > height - EDGE_SCROLL_PX {
        pan_y = -1.0;
    }

    if pan_x != 0.0 || pan_y != 0.0 {
        let mut max_pos: Vec2 = map_size
            .get_single()
            .ok()
            .copied()
            .unwrap_or_default()
            .into();
        max_pos *= settings.asset_quality.tile_size() / 2.0;
        // Let the camera go half the width/height past the edge of the map so centering is always
        // possible. Note that we divide by 4 since this value is for the edges of a rect centered
        // at (0, 0).
        // TODO(tec27): We can likely make this smaller without causing real issues?
        // TODO(tec27): Need to adjust this based on what a window pixel is with camera scale as
        // well.
        max_pos += Vec2::new(width / 4.0, height / 4.0);

        let (mut transform, projection) = camera_query.single_mut();
        let scale = projection.scale;
        transform.translation += Vec3::new(
            pan_x * MOUSE_PAN_SPEED * time.delta_seconds() * scale,
            pan_y * MOUSE_PAN_SPEED * time.delta_seconds() * scale,
            0.0,
        );
        transform.translation = transform
            .translation
            .clamp(-max_pos.extend(0.0), max_pos.extend(0.0));
    }
}
