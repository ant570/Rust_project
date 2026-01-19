use crate::GameState;
use crate::scenes::menu::OnMenuScreen;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Component, PartialEq, Eq)]
pub enum MenuHtpButtonAction {
    Back,
}

pub fn spawn_htp(mut commands: Commands) {
    commands
        .spawn((
            OnMenuScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::from(BLACK)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("HOW TO PLAY"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("PLAYER 1"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
            ));

            let p1_instructions = ["Move: WSAD", "Attack: F"];

            for text in p1_instructions {
                parent.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            }

            parent.spawn((
                Text::new("PLAYER 2"),
                TextFont {
                    font_size: 35.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));

            let p2_instructions = ["Move: Arrows", "Attack: Dot (.)"];

            for text in p2_instructions {
                parent.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            }
            parent.spawn((
                Text::new("Task: Collect coins and avoid damage!"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("First to reach the win score wins."),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("You can gain points by collecting coins and hitting the opponent."),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("You can set the win score and scoring rules in Settings."),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("ESC - Pause / Back to Menu"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::from(GOLD)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));

            parent
                .spawn((
                    Button,
                    MenuHtpButtonAction::Back,
                    OnMenuScreen,
                    Node {
                        width: Val::Px(500.0),
                        height: Val::Px(80.0),
                        margin: UiRect::top(Val::Px(40.0)),
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
                        Text::new("BACK"),
                        TextFont {
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(Color::from(GOLD)),
                    ));
                });
        });
}

pub fn htp_action(
    interaction_query: Query<
        (&Interaction, &MenuHtpButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
    current_state: Res<State<crate::scenes::game_state::GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed && *action == MenuHtpButtonAction::Back {
            match current_state.get() {
                GameState::HowToPlay1 => {
                    if *action == MenuHtpButtonAction::Back {
                        next_state.set(GameState::StartMenu);
                    }
                }
                GameState::HowToPlay2 => {
                    if *action == MenuHtpButtonAction::Back {
                        next_state.set(GameState::Paused);
                    }
                }
                _ => {}
            }
        }
    }
}
