use bevy::prelude::*;
use bevy::color::palettes::css::*;
use crate::scenes::menu::OnMenuScreen;
use crate::GameState;

#[derive(Component)]
pub enum FinishMenuButtonAction {
    Restart,
    Exit,
}

pub fn spawn_finish_menu(mut commands: Commands) {
    commands.spawn((
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
            Text::new("GAME Finished"),
            TextFont { font_size: 100.0, ..default() },
            TextColor(Color::from(GOLD)),
            Node { margin: UiRect::bottom(Val::Px(50.0)), ..default() },
        ));

        let buttons = [
            ("MAIN MENU", FinishMenuButtonAction::Restart),
            ("EXIT", FinishMenuButtonAction::Exit),
        ];

        for (label, action) in buttons {
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
            .with_children(|btn| {
                btn.spawn((
                    Text::new(label),
                    TextFont { font_size: 30.0, ..default() },
                    TextColor(Color::from(GOLD)),
                ));
            });
        }
    });
}

pub fn finish_menu_action(
    interaction_query: Query<
        (&Interaction, &FinishMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                FinishMenuButtonAction::Restart => {
                    next_state.set(GameState::StartMenu);
                }
                FinishMenuButtonAction::Exit => {
                    exit.write(AppExit::Success);
                }
                _ => {}
            }
        }
    }
}