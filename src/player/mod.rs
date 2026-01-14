use bevy::prelude::*;
pub struct PlatformerGamePlugin;
pub mod attack;
pub mod movement;
pub mod player;
pub mod position;
pub mod score;
pub mod spawn;

impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_players)
            .add_systems(Update, movement::keyboard_input)
            .add_systems(Update, attack::stick_attack)
            .add_systems(Update, spawn::check_player_fall)
            .add_systems(Update, score::update_score_text);
    }
}
