use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

mod game;

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
        .add_plugins(crate::game::PlatformerGamePlugin)
        .run();
}
