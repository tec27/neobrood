use bevy::prelude::*;
use bevy::window::CursorGrabMode;

/// How far from the edge of the screen the mouse needs to be to start scrolling, in pixels.
const EDGE_SCROLL_PX: f32 = 4.0;
const MOUSE_PAN_SPEED: f32 = 3000.0;

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPanLocked>()
            .add_startup_system(camera_control_setup)
            .add_system(camera_control);
    }
}

/// A resource that indicates whether the camera should be locked to its current position. This
/// typically occurs because a drag selection is in progress.
#[derive(Resource, Default, Debug)]
pub struct CameraPanLocked(pub bool);

fn camera_control_setup(mut windows: ResMut<Windows>) {
    // TODO(tec27): This probably needs to be redone when the window regains focus
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_grab_mode(CursorGrabMode::Confined);
}

#[derive(Default)]
struct CameraControlState {
    last_pos: Vec2,
}

fn camera_control(
    mut state: Local<CameraControlState>,
    windows: Res<Windows>,
    time: Res<Time>,
    camera_pan_locked: Res<CameraPanLocked>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    // TODO(tec27): implement arrow key scrolling + middle mouse panning as well
    let window = windows.get_primary().unwrap();
    let mouse_position = window.cursor_position().unwrap_or(state.last_pos);
    state.last_pos = mouse_position;

    if camera_pan_locked.0 {
        return;
    }

    let width = window.width();
    let height = window.height();

    let mut scroll_x = 0.0;
    let mut scroll_y = 0.0;

    if mouse_position.x < EDGE_SCROLL_PX {
        scroll_x = -1.0;
    } else if mouse_position.x > width - EDGE_SCROLL_PX {
        scroll_x = 1.0;
    }

    if mouse_position.y < EDGE_SCROLL_PX {
        scroll_y = -1.0;
    } else if mouse_position.y > height - EDGE_SCROLL_PX {
        scroll_y = 1.0;
    }

    if scroll_x != 0.0 || scroll_y != 0.0 {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation += Vec3::new(
            scroll_x * MOUSE_PAN_SPEED * time.delta_seconds(),
            scroll_y * MOUSE_PAN_SPEED * time.delta_seconds(),
            0.0,
        );
    }
}
