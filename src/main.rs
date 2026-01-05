use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

mod world;
mod player;
mod physics;
 
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
        .add_plugins(crate::physics::PlatformerGamePluginCollision)
        .run();
}
