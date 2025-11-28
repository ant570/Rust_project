use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

mod world;
mod player;
use crate::player::position::Position2;
use crate::player::spawn::spawn_player;
use crate::player::spawn::player_movement;
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "XDD".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(crate::world::PlatformerGamePlugin)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .run();

    let pos = Position2::new(10.0, 20.0);
    pos.to_str();
}
