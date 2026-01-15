use bevy::prelude::*;

#[derive(Component)]
pub struct OnMenuScreen;
pub fn start_menu(mut commands: Commands){
    println!("Menu startuje!");
    commands.spawn((
        OnMenuScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 0.2)),
    ))
    .with_children(|parent| {
        parent.spawn((
            OnMenuScreen,
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                OnMenuScreen,
                Text::new("Start Game"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn menu_action(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Start Game!");
            next_state.set(crate::scenes::game_state::GameState::Playing);
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands, 
    query: Query<Entity, With<OnMenuScreen>>
) {
    println!("Exiting Menu");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}