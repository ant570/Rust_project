use crate::player::player::Player;
use crate::player::spawn::ScoreText;
use bevy::prelude::*;

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
