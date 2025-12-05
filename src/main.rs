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
        .add_plugins(crate::player::PlatformerGamePlugin)
        .run();
}
