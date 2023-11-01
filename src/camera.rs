use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};

/// How far from the edge of the screen the mouse needs to be to start scrolling, in pixels.
const EDGE_SCROLL_PX: f32 = 4.0;
const MOUSE_PAN_SPEED: f32 = 3000.0;
/// How fast the camera zooms in/out from scrolling.
const MOUSE_ZOOM_SPEED: f32 = 0.5;

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPanLocked>()
            .add_systems(Update, (handle_window_focus, camera_control));
    }
}

/// A resource that indicates whether the camera should be locked to its current position. This
/// typically occurs because a drag selection is in progress.
#[derive(Resource, Default, Debug)]
pub struct CameraPanLocked(pub bool);

fn handle_window_focus(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventReader<WindowFocused>,
) {
    for event in events.iter() {
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
) {
    // TODO(tec27): implement arrow key scrolling + middle mouse panning as well
    let window = window.get_single().unwrap();
    let mouse_position = window.cursor_position().unwrap_or(state.last_pos);
    state.last_pos = mouse_position;

    if camera_pan_locked.0 {
        return;
    }

    let scroll_delta = scroll_events.iter().fold(0.0, |acc, event| {
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
        let (mut transform, projection) = camera_query.single_mut();
        let scale = projection.scale;
        transform.translation += Vec3::new(
            pan_x * MOUSE_PAN_SPEED * time.delta_seconds() * scale,
            pan_y * MOUSE_PAN_SPEED * time.delta_seconds() * scale,
            0.0,
        );
    }
}
