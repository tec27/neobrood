use crate::camera::CameraPanLocked;
use crate::states::AppState;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct DragSelectionPlugin;

impl Plugin for DragSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), drag_selection_setup)
            .add_systems(OnExit(AppState::InGame), drag_selection_cleanup)
            .add_systems(Update, drag_selection.run_if(in_state(AppState::InGame)));
    }
}

/// How far the mouse cursor can move with the mouse down before it is considered a drag
const DRAG_SLOP_PX: f32 = 4.0;

#[derive(Default)]
struct DragSelectionState {
    mouse_down: bool,
    mouse_down_pos: Vec2,
}

impl DragSelectionState {
    fn is_dragging(&self, mouse_pos: Vec2) -> bool {
        self.mouse_down && mouse_pos.distance_squared(self.mouse_down_pos) >= DRAG_SLOP_PX
    }
}

#[derive(Component)]
struct DragSelectionBox;

fn drag_selection_setup(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: Color::rgba(0.2, 0.8, 0.4, 0.8).into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        DragSelectionBox,
    ));
}

fn drag_selection_cleanup(mut commands: Commands, query: Query<Entity, With<DragSelectionBox>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn drag_selection(
    mut state: Local<DragSelectionState>,
    mut camera_pan_locked: ResMut<CameraPanLocked>,
    mut mouse_reader: EventReader<MouseButtonInput>,
    window: Query<&Window, (With<PrimaryWindow>, Without<DragSelectionBox>)>,
    mut drag_box_query: Query<(&mut Style, &mut Visibility), With<DragSelectionBox>>,
) {
    let window = window.get_single().unwrap();
    let mouse_pos = window.cursor_position().unwrap_or_default();

    for event in mouse_reader.read() {
        if event.button != MouseButton::Left {
            continue;
        }

        match event.state {
            ButtonState::Pressed => {
                state.mouse_down = true;
                state.mouse_down_pos = mouse_pos;
                camera_pan_locked.0 = true;
            }
            ButtonState::Released => {
                #[allow(clippy::if_same_then_else)]
                if state.is_dragging(mouse_pos) {
                    // TODO(tec27): complete drag
                } else {
                    // TODO(tec27): This was just a click, maybe handle this here too? Rename the
                    // system/state in that case probably
                }
                state.mouse_down = false;
                camera_pan_locked.0 = false;
            }
        }
    }

    let (mut box_style, mut box_visibility) = drag_box_query.single_mut();
    if state.is_dragging(mouse_pos) {
        *box_visibility = Visibility::Visible;

        let (left, right) = if mouse_pos.x < state.mouse_down_pos.x {
            (mouse_pos.x, window.width() - state.mouse_down_pos.x)
        } else {
            (state.mouse_down_pos.x, window.width() - mouse_pos.x)
        };

        let (top, bottom) = if mouse_pos.y < state.mouse_down_pos.y {
            (mouse_pos.y, window.height() - state.mouse_down_pos.y)
        } else {
            (state.mouse_down_pos.y, window.height() - mouse_pos.y)
        };

        box_style.left = Val::Px(left);
        box_style.right = Val::Px(right);
        box_style.top = Val::Px(top);
        box_style.bottom = Val::Px(bottom);
    } else {
        *box_visibility = Visibility::Hidden;
    }
}
