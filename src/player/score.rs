use crate::player::core::Player;
use crate::player::spawn::ScoreText;
use bevy::prelude::*;

//Aktualizacja wyświetlanej punktacji
pub fn update_score_text(
    query: Query<(&Player, &Children)>,
    mut text_query: Query<&mut Text2d, With<ScoreText>>,
) {
    for (player, children) in query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = player.points.to_string();
            }
        }
    }
}

//Sprawdzenie czy któryś z graczy osiągnął zwycięzki wynik
pub fn check_win_condition(
    player_query: Query<&Player>,
    settings: Res<crate::scenes::menu::settings::Settings>,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
) {
    for player in player_query.iter() {
        if player.points >= settings.win_score {
            next_state.set(crate::scenes::game_state::GameState::Finished);
        }
    }
}
