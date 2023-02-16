use crate::camera::CameraPanLocked;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

pub struct DragSelectionPlugin;

impl Plugin for DragSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(drag_selection_setup)
            .add_system(drag_selection);
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
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::all(Val::Px(0.0)),
                    ..default()
                },
                visibility: Visibility::INVISIBLE,
                ..default()
            },
            DragSelectionBox,
        ))
        .with_children(|parent| {
            // I don't know of a way to just render borders, so we make our own border by putting a
            // box at each edge? Probably not that efficient given it has a layout pass but
            // until I find a better way, this'll do I guess.
            let color = Color::rgba(0.2, 0.8, 0.4, 0.8);
            let box_size = Val::Px(2.0);
            // left
            parent.spawn(NodeBundle {
                background_color: color.into(),
                style: Style {
                    size: Size::new(box_size, Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
            // right
            parent.spawn(NodeBundle {
                background_color: color.into(),
                style: Style {
                    size: Size::new(box_size, Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
            // top
            parent.spawn(NodeBundle {
                background_color: color.into(),
                style: Style {
                    size: Size::new(Val::Percent(100.0), box_size),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
            // bottom
            parent.spawn(NodeBundle {
                background_color: color.into(),
                style: Style {
                    size: Size::new(Val::Percent(100.0), box_size),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
        });
}

fn drag_selection(
    mut state: Local<DragSelectionState>,
    windows: Res<Windows>,
    mut camera_pan_locked: ResMut<CameraPanLocked>,
    mut mouse_reader: EventReader<MouseButtonInput>,
    mut drag_box_query: Query<(&mut Style, &mut Visibility), With<DragSelectionBox>>,
) {
    let window = windows.get_primary().unwrap();
    let mut mouse_pos = window.cursor_position().unwrap_or_default();
    // Flip the y coordinates to match the Bevy UI coords
    mouse_pos.y = window.height() - mouse_pos.y;
    let mouse_pos = mouse_pos;

    for event in mouse_reader.iter() {
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
        box_visibility.is_visible = true;

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

        box_style.position = UiRect {
            left: Val::Px(left),
            right: Val::Px(right),
            top: Val::Px(top),
            bottom: Val::Px(bottom),
        }
    } else {
        box_visibility.is_visible = false;
    }
}
