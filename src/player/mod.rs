use bevy::prelude::*;
pub struct PlatformerGamePlugin;
pub mod spawn;
pub mod position;


impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn::spawn_players)
            .add_systems(Update, spawn::player_movement);
    }
}