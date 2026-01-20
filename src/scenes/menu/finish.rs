use crate::GameState;
use crate::player::core::Control;
use crate::player::core::Player;
use crate::scenes::menu::OnMenuScreen;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

type FinishInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static FinishMenuButtonAction),
    (Changed<Interaction>, With<Button>),
>;

type WorldEntitiesQuery<'w, 's> =
    Query<'w, 's, Entity, (Without<Camera>, Without<DirectionalLight>, Without<Window>)>;

#[derive(Component)]
//Typy przycisków
pub enum FinishMenuButtonAction {
    Restart,
    Exit,
}

//Menu zakończenia gry
pub fn spawn_finish_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    players_query: Query<&Player>,
) {
    //Pobranie punktów graczy
    let mut p1_points = 0;
    let mut p2_points = 0;
    for player in &players_query {
        match player.control {
            Control::Wasd => p1_points = player.points,
            Control::Arrows => p2_points = player.points,
        }
    }

    //Spawn menu końcowego
    commands
        .spawn((
            OnMenuScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::from(BLACK)),
        ))
        .with_children(|parent| {
            // lewy gracz (lewa strona)
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|p1| {
                    p1.spawn((
                        ImageNode::new(asset_server.load("players/player2.png")),
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            ..default()
                        },
                    ));
                    //spawn punktów
                    p1.spawn((
                        Text::new(format!("SCORE: {}", p1_points)),
                        TextFont {
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor(Color::from(GOLD)),
                        Node {
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        },
                    ));
                });

            // (środek) - ogłoszenie zwycięzcy
            let winner = if p1_points > p2_points {
                "LEFT PLAYER WINS!"
            } else if p2_points > p1_points {
                "RIGHT PLAYER WINS!"
            } else {
                "     IT'S A TIE!     "
            };

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|mid| {
                    mid.spawn((
                        Text::new(winner),
                        TextFont {
                            font_size: 70.0,
                            ..default()
                        },
                        TextColor(Color::from(GOLD)),
                        Node {
                            margin: UiRect::bottom(Val::Px(40.0)),
                            ..default()
                        },
                    ));

                    //spawn przycisków
                    let buttons = [
                        ("MAIN MENU", FinishMenuButtonAction::Restart),
                        ("EXIT", FinishMenuButtonAction::Exit),
                    ];

                    for (label, action) in buttons {
                        mid.spawn((
                            Button,
                            action,
                            Node {
                                width: Val::Px(500.0),
                                height: Val::Px(80.0),
                                margin: UiRect::vertical(Val::Px(20.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            BorderColor::all(Color::from(GOLD)),
                            BackgroundColor(Color::from(BLACK)),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new(label),
                                TextFont {
                                    font_size: 25.0,
                                    ..default()
                                },
                                TextColor(Color::from(GOLD)),
                            ));
                        });
                    }
                });

            // prawy gracz
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|p2| {
                    p2.spawn((
                        ImageNode::new(asset_server.load("players/player1.png")),
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            ..default()
                        },
                    ));

                    //spawn punktów
                    p2.spawn((
                        Text::new(format!("SCORE: {}", p2_points)),
                        TextFont {
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor(Color::from(GOLD)),
                        Node {
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn finish_menu_action(
    interaction_query: FinishInteractionQuery,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
    query: WorldEntitiesQuery,
) {
    for (interaction, action) in interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                FinishMenuButtonAction::Restart => {
                    //usunięcie wszystkich obiektów z gry
                    for entity in &query {
                        commands.entity(entity).try_despawn();
                    }
                    next_state.set(GameState::StartMenu);
                }
                FinishMenuButtonAction::Exit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}
