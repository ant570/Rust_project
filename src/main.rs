use bevy::prelude::*;
mod game;
use crate::game::position::Position2;
use crate::game::player::spawn_player;
 use crate::game::player::player_movement;
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::game::PlatformerGamePlugin)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .run();

    let pos = Position2::new(10.0, 20.0);
    pos.to_str();
}