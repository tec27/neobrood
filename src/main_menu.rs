use std::path::PathBuf;

use bevy::{app::AppExit, prelude::*};

use crate::{
    ecs::despawn_all,
    maps::{load_map, CurrentMap},
    settings::GameSettings,
    states::AppState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup)
            .add_systems(OnExit(AppState::Menu), despawn_all::<OnMainMenu>)
            .add_systems(
                Update,
                (actions, update_button_colors, map_drag_and_drop).run_if(in_state(AppState::Menu)),
            );
    }
}

#[derive(Component)]
struct OnMainMenu;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Debug)]
enum MenuAction {
    LoadLostTemple,
    Quit,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sofia = asset_server.load("fonts/SofiaSans-Bold.ttf");
    let inter = asset_server.load("fonts/Inter-Regular.ttf");

    let button_style = Style {
        width: Val::Px(240.0),
        height: Val::Px(64.0),
        margin: UiRect::all(Val::Px(16.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: inter.clone(),
        font_size: 32.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenu,
        ))
        .with_children(|parent| {
            parent
                // TODO(tec27): This seems to get a transform with translation of (0.5, 0.5) for
                // some reason which makes the text blurry, need to figure out why/fix it
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "neobrood",
                            TextStyle {
                                font: sofia,
                                font_size: 80.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(24.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(
                        TextBundle::from_section(
                            "drag and drop a map to start, or use the menu below",
                            TextStyle {
                                font: inter,
                                font_size: 24.0,
                                color: Color::rgb(0.7, 0.7, 0.7),
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(40.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BackgroundColor(NORMAL_BUTTON),
                                ..default()
                            },
                            MenuAction::LoadLostTemple,
                        ))
                        .with_children(|parent| {
                            parent.spawn((TextBundle::from_section(
                                "load lost temple",
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
                            MenuAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("quit", button_text_style.clone()));
                        });
                });
        });
}

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

fn actions(
    query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
    mut current_map: ResMut<CurrentMap>,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuAction::LoadLostTemple => load_map(
                    &PathBuf::from("lt.scm"),
                    &mut current_map,
                    &mut next_state,
                    &asset_server,
                    &settings,
                ),
                MenuAction::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}

fn map_drag_and_drop(
    mut drop_events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
    mut current_map: ResMut<CurrentMap>,
    mut next_state: ResMut<NextState<AppState>>,
    settings: Res<GameSettings>,
) {
    for event in drop_events.read() {
        let FileDragAndDrop::DroppedFile { path_buf, .. } = event else {
            continue;
        };

        let extension = path_buf.extension().map_or("".into(), |s| {
            s.to_ascii_lowercase().to_string_lossy().to_string()
        });
        if extension == "scm" || extension == "scx" {
            load_map(
                path_buf,
                &mut current_map,
                &mut next_state,
                &asset_server,
                &settings,
            )
        }
    }
}
