use crate::camera::CameraPanLocked;
use crate::gamedata::ConstructTypeId;
use crate::gameplay::InGameMenuState;
use crate::maps::game_map::{GameMap, GameMapSize, LOGIC_TILE_SIZE};
use crate::maps::position::Position;
use crate::settings::GameSettings;
use crate::states::AppState;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::smallvec::SmallVec;
use bevy::window::PrimaryWindow;

use super::constructs::OwnedConstruct;
use super::players::{ControlledPlayer, PlayerEntities};

pub struct DragSelectionPlugin;

impl Plugin for DragSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectEvent>()
            .add_systems(OnEnter(AppState::InGame), drag_selection_setup)
            .add_systems(OnExit(AppState::InGame), drag_selection_cleanup)
            .add_systems(
                Update,
                (selection_input, apply_selection, update_locally_selected)
                    .chain()
                    .run_if(
                        in_state(AppState::InGame).and_then(in_state(InGameMenuState::Disabled)),
                    ),
            );
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

// TODO(tec27): Handle modifiers, e.g. shift-select should add to the existing selection
/// Event fired when a selection is completed, specified by logical map coordinates.
#[derive(Event, Debug, Copy, Clone)]
pub enum SelectEvent {
    Click(Position),
    Drag(DragSelectEvent),
}

#[derive(Event, Debug, Copy, Clone)]
pub struct DragSelectEvent {
    pub start: Position,
    pub end: Position,
}

#[derive(Component)]
struct DragSelectionBox;

/// Set up the drag selection UI.
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

/// Clean up the drag selection UI.
fn drag_selection_cleanup(mut commands: Commands, query: Query<Entity, With<DragSelectionBox>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn selection_input(
    mut state: Local<DragSelectionState>,
    mut camera_pan_locked: ResMut<CameraPanLocked>,
    mut mouse_reader: EventReader<MouseButtonInput>,
    mut drag_box_query: Query<(&mut Style, &mut Visibility), With<DragSelectionBox>>,
    mut select_event_writer: EventWriter<SelectEvent>,
    window: Query<&Window, (With<PrimaryWindow>, Without<DragSelectionBox>)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    map: Query<&GameMapSize, With<GameMap>>,
    settings: Res<GameSettings>,
) {
    let window = window.single();
    let mouse_pos = window.cursor_position().unwrap_or_default();

    for event in mouse_reader.read() {
        if event.button != MouseButton::Left {
            continue;
        }

        match event.state {
            ButtonState::Pressed => {
                // TODO(tec27): Only handle this if the mouse is over the map (e.g. not over the
                // UI)
                state.mouse_down = true;
                state.mouse_down_pos = mouse_pos;
                camera_pan_locked.0 = true;
            }
            ButtonState::Released => {
                let map_size = map.single();
                let half_map_size = Vec2::from(map_size) / 2.0;
                let tile_size = settings.asset_quality.tile_size();
                // Converts the world coordinates to logical map coordinates
                let convert_pos = |mut pos: Vec2| -> IVec2 {
                    pos /= tile_size;
                    pos += half_map_size + 0.5;
                    pos.y = map_size.height as f32 - pos.y;
                    pos *= LOGIC_TILE_SIZE as f32;

                    IVec2::new(pos.x.round() as i32, pos.y.round() as i32)
                };
                let (cam_transform, cam) = camera_query.single();

                if state.is_dragging(mouse_pos) {
                    // Convert the viewport coordinates to world coordinates
                    let start = cam
                        .viewport_to_world_2d(cam_transform, state.mouse_down_pos)
                        .unwrap();
                    let end = cam.viewport_to_world_2d(cam_transform, mouse_pos).unwrap();

                    let clamp_to_map = |pos: IVec2| -> IVec2 {
                        pos.clamp(
                            IVec2::ZERO,
                            IVec2::new(
                                map_size.width as i32 * LOGIC_TILE_SIZE,
                                map_size.height as i32 * LOGIC_TILE_SIZE,
                            ),
                        )
                    };

                    let start = clamp_to_map(convert_pos(start)).into();
                    let end = clamp_to_map(convert_pos(end)).into();
                    select_event_writer.send(SelectEvent::Drag(DragSelectEvent { start, end }));
                } else {
                    // TODO(tec27): Would it be better to use the mouse down position? The average?
                    let pos =
                        convert_pos(cam.viewport_to_world_2d(cam_transform, mouse_pos).unwrap());
                    let map_rect = IRect::from_corners(
                        IVec2::ZERO,
                        IVec2::new(
                            map_size.width as i32 * LOGIC_TILE_SIZE,
                            map_size.height as i32 * LOGIC_TILE_SIZE,
                        ),
                    );
                    if map_rect.contains(pos) {
                        select_event_writer.send(SelectEvent::Click(pos.into()));
                    }
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

/// Component that stores the currently selected entities for a [Player].
#[derive(Component, Debug, Default)]
pub struct SelectedEntities(pub SmallVec<[Entity; 12]>);

// TODO(tec27): We might need to rework this for team games/obs/replays to be able to show selection
// circles for other players. Might make sense to have this contain a bitfield?
/// Marker component for a construct that is currently selected by the [ControllerPlayer]. This is
/// expected to have a [Parent] component pointing to the actual construct entity.
#[derive(Component)]
pub struct LocallySelected;

fn apply_selection(
    mut drag_events: EventReader<SelectEvent>,
    mut controlled_player: Query<(Entity, &mut SelectedEntities), With<ControlledPlayer>>,
    constructs: Query<(
        Entity,
        &Position,
        &ConstructTypeId,
        &Visibility,
        Option<&OwnedConstruct>,
    )>,
    player_entities: Res<PlayerEntities>,
) {
    let Ok((controlled_player, mut selected_entities)) = controlled_player.get_single_mut() else {
        // No locally-controlled player so drag selection can't be done
        // TODO(tec27): Figure out how observers should work with this
        return;
    };
    let Some(controlled_player) = player_entities.player_num_for(controlled_player) else {
        warn!("Couldn't find controlled player in PlayerEntities");
        return;
    };

    for event in drag_events.read() {
        match event {
            SelectEvent::Click(pos) => {
                handle_click_selection(
                    pos.into(),
                    controlled_player,
                    &constructs,
                    &mut selected_entities,
                );
            }
            SelectEvent::Drag(event) => {
                handle_drag_selection(
                    event.start.into(),
                    IRect::from_corners(event.start.into(), event.end.into()),
                    controlled_player,
                    &constructs,
                    &mut selected_entities,
                );
            }
        }
    }
}

fn handle_click_selection(
    click_pos: IVec2,
    controlled_player: u8,
    constructs: &Query<(
        Entity,
        &Position,
        &ConstructTypeId,
        &Visibility,
        Option<&OwnedConstruct>,
    )>,
    selected_entities: &mut Mut<SelectedEntities>,
) {
    // All constructs that are visible and contain the click
    let contained_constructs = constructs
        .iter()
        .filter(|(_, &pos, &ty, &vis, _)| {
            vis != Visibility::Hidden && ty.def().bounds.at_pos(pos.into()).contains(click_pos)
        })
        .collect::<Vec<_>>();
    let owned = {
        // All constructs from above that are also owned by the current player
        let owned = contained_constructs
            .iter()
            .copied()
            .filter(|(_, _, _, _, oc)| match oc {
                Some(owner) => owner.0 == controlled_player,
                None => false,
            })
            .collect::<Vec<_>>();
        // All constructs from above that are also units
        let owned_units = owned
            .iter()
            .copied()
            .filter(|(_, _, &ty, _, _)| ty.is_unit())
            .collect::<Vec<_>>();

        // If there are any units, we prefer those, otherwise we fall back to buildings
        if !owned_units.is_empty() {
            owned_units
        } else {
            owned
        }
    };

    // If there are any owned constructs, we select from those, otherwise we select the highest
    // priority construct
    let mut selectable = if !owned.is_empty() {
        owned
    } else {
        contained_constructs
    };

    // Sort by Euclidean distnace (squared) from the click
    selectable.sort_by_cached_key(|(_, pos, _, _, _)| {
        (pos.x - click_pos.x).pow(2) + (pos.y - click_pos.y).pow(2)
    });

    // TODO(tec27): For click selection it probably makes more sense to prefer things at the higher
    // "layer" (e.g. prefer flying units over ground units?). Potentially we should just project a
    // ray from the camera through the click and select the first thing it hits?

    selected_entities.0.clear();
    selected_entities
        .0
        .extend(selectable.first().map(|(entity, _, _, _, _)| *entity));
}

fn handle_drag_selection(
    drag_start: IVec2,
    drag_rect: IRect,
    controlled_player: u8,
    constructs: &Query<(
        Entity,
        &Position,
        &ConstructTypeId,
        &Visibility,
        Option<&OwnedConstruct>,
    )>,
    selected_entities: &mut Mut<SelectedEntities>,
) {
    // All constructs that are visible and within the drag
    let contained_constructs = constructs
        .iter()
        .filter(|(_, &pos, &ty, &vis, _)| {
            vis != Visibility::Hidden
                && !drag_rect
                    .intersect(ty.def().bounds.at_pos(pos.into()))
                    .is_empty()
        })
        .collect::<Vec<_>>();
    let owned = {
        // All constructs from above that are also owned by the current player
        let owned = contained_constructs
            .iter()
            .copied()
            .filter(|(_, _, _, _, oc)| match oc {
                Some(owner) => owner.0 == controlled_player,
                None => false,
            })
            .collect::<Vec<_>>();
        // All constructs from above that are also units
        let owned_units = owned
            .iter()
            .copied()
            .filter(|(_, _, &ty, _, _)| ty.is_unit())
            .collect::<Vec<_>>();

        // If there are any units, we prefer those, otherwise we fall back to buildings
        if !owned_units.is_empty() {
            owned_units
        } else {
            owned
        }
    };

    // If there are any owned constructs, we select from those (up to 12), otherwise we select
    // the highest priority construct
    let (mut selectable, max_count) = if !owned.is_empty() {
        (owned, 12)
    } else {
        let owned_by_others = contained_constructs
            .iter()
            .copied()
            .filter(|(_, _, _, _, oc)| oc.is_some())
            .collect::<Vec<_>>();
        if !owned_by_others.is_empty() {
            (owned_by_others, 1)
        } else {
            (contained_constructs, 1)
        }
    };

    // Sort by Euclidean distnace (squared) from the start of the selection
    selectable.sort_by_cached_key(|(_, pos, _, _, _)| {
        (pos.x - drag_start.x).pow(2) + (pos.y - drag_start.y).pow(2)
    });

    selected_entities.0.clear();
    selected_entities.0.extend(
        selectable
            .iter()
            .take(max_count)
            .map(|(entity, _, _, _, _)| *entity),
    );
}

fn update_locally_selected(
    mut commands: Commands,
    controlled_player: Query<&SelectedEntities, With<ControlledPlayer>>,
    last_selected: Query<(Entity, &Parent), With<LocallySelected>>,
) {
    for (e, p) in last_selected.iter() {
        commands.entity(p.get()).remove_children(&[e]);
        commands.entity(e).despawn();
    }

    for selected in controlled_player.iter() {
        for &e in selected.0.iter() {
            let child = commands
                .spawn((
                    LocallySelected,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.0, 1.0, 0.0, 0.5),
                            custom_size: Some(Vec2::new(32.0, 32.0)),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .id();
            commands.entity(e).add_child(child);
        }
    }
}
