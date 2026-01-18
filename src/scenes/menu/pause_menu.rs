use bevy::prelude::*;
use bevy::color::palettes::css::*;
use bevy::app::AppExit;
use crate::scenes::menu::OnMenuScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Continue,
    FinishNow,
    Restart,
    HowToPlay,
    Settings,
    Exit,
}



pub fn pause_menu(mut commands: Commands){
    commands.spawn((
        OnMenuScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::from(BLACK)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Paused"),
            TextFont { font_size: 100.0, ..default() },
            TextColor(Color::from(GOLD)),
            Node { margin: UiRect::bottom(Val::Px(50.0)), ..default() },
        ));
        let button_labels = [
            ("CONTINUE", MenuButtonAction::Continue),
            ("FINISH NOW", MenuButtonAction::FinishNow),
            ("RESTART", MenuButtonAction::Restart),
            ("HOW TO PLAY", MenuButtonAction::HowToPlay),
            ("SETTINGS", MenuButtonAction::Settings),
            ("EXIT", MenuButtonAction::Exit),
        ];

        for (label, action) in button_labels {
            parent.spawn((
                Button,
                action,
                OnMenuScreen,
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
            .with_children(|button_parent| {
                button_parent.spawn((
                    Text::new(label),
                    TextFont { font_size: 50.0, ..default() },
                    TextColor(Color::from(GOLD)),
                ));
            });
        }
    });
}

pub fn pause_menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
    query: Query<Entity, (Without<Camera>, Without<DirectionalLight>, Without<Window>)>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Continue => {
                    next_state.set(crate::scenes::game_state::GameState::Playing);
                }
                MenuButtonAction::FinishNow => {
                    next_state.set(crate::scenes::game_state::GameState::Finished);
                }
                MenuButtonAction::Restart => {
                    for entity in &query {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(crate::scenes::game_state::GameState::StartMenu);
                }
                MenuButtonAction::HowToPlay => {
                    next_state.set(crate::scenes::game_state::GameState::HowToPlay2);   
                }
                MenuButtonAction::Settings => {
                    next_state.set(crate::scenes::game_state::GameState::SettingsPause);  
                }
                MenuButtonAction::Exit => {
                    exit.write(AppExit::Success);       
                }
            }
        }
    }
}



