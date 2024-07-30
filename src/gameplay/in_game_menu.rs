use bevy::{app::AppExit, prelude::*};

use crate::{
    ecs::{despawn_all, log_transitions},
    fonts::FONT_BODY,
    states::AppState,
};

use super::gizmos::ConstructGizmos;

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InGameMenuState>()
            .add_systems(Update, log_transitions::<InGameMenuState>)
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                OnExit(AppState::InGame),
                (despawn_all::<OnInGameMenu>, cleanup),
            )
            .add_systems(Update, handle_keys.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                update_button_colors.run_if(
                    in_state(AppState::InGame).and_then(not(in_state(InGameMenuState::Disabled))),
                ),
            )
            .add_systems(OnEnter(InGameMenuState::General), setup_general)
            .add_systems(
                OnExit(InGameMenuState::General),
                despawn_all::<OnGeneralMenu>,
            )
            .add_systems(
                Update,
                general_actions.run_if(in_state(InGameMenuState::General)),
            );
    }
}

#[derive(Component)]
pub struct OnInGameMenu;

#[derive(Component)]
pub struct OnGeneralMenu;

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InGameMenuState {
    #[default]
    Disabled,
    General,
}

fn setup(mut menu_state: ResMut<NextState<InGameMenuState>>) {
    menu_state.set(InGameMenuState::Disabled);
}

fn cleanup(mut menu_state: ResMut<NextState<InGameMenuState>>) {
    menu_state.set(InGameMenuState::Disabled);
}

fn handle_keys(
    cur_state: Res<State<InGameMenuState>>,
    mut next_state: ResMut<NextState<InGameMenuState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F10) {
        next_state.set(match cur_state.get() {
            InGameMenuState::Disabled => InGameMenuState::General,
            _ => InGameMenuState::Disabled,
        })
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn update_button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg) in &mut interaction_query {
        bg.0 = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
    }
}

#[derive(Component, Debug)]
enum GeneralMenuAction {
    ToggleDevTools,
    EndGame,
    Quit,
}

fn setup_general(mut commands: Commands, asset_server: Res<AssetServer>) {
    let body = asset_server.load(FONT_BODY);

    let button_style = Style {
        width: Val::Px(240.0),
        height: Val::Px(64.0),
        margin: UiRect::all(Val::Px(8.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: body.clone(),
        font_size: 32.0,
        color: Color::srgb(0.9, 0.9, 0.9),
    };

    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnInGameMenu,
            OnGeneralMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: BackgroundColor(NORMAL_BUTTON),
                        ..default()
                    },
                    GeneralMenuAction::ToggleDevTools,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "toggle dev tools",
                        button_text_style.clone(),
                    ),));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: BackgroundColor(NORMAL_BUTTON),
                        ..default()
                    },
                    GeneralMenuAction::EndGame,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "end game",
                        button_text_style.clone(),
                    ),));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: BackgroundColor(NORMAL_BUTTON),
                        ..default()
                    },
                    GeneralMenuAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section("quit", button_text_style.clone()),));
                });
        });
}

fn general_actions(
    query: Query<(&Interaction, &GeneralMenuAction), (Changed<Interaction>, With<Button>)>,
    mut gizmo_store: ResMut<GizmoConfigStore>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                GeneralMenuAction::ToggleDevTools => {
                    let (config, _) = gizmo_store.config_mut::<ConstructGizmos>();
                    config.enabled ^= true;
                }
                GeneralMenuAction::EndGame => {
                    next_app_state.set(AppState::Menu);
                }
                GeneralMenuAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}
