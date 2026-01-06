use bevy::prelude::*;
pub struct PlatformerGamePlugin;
pub mod spawn;
pub mod position;
pub mod movement;
pub mod player;
pub mod score;

impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn::spawn_players)
            .add_systems(Update, movement::keyboard_input)
            .add_systems(Update, score::update_score_text);
    }
}