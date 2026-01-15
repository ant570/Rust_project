use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};
use crate::scenes::game_state::GameState;

mod audio;
mod physics;
mod player;
mod scenes;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "XDD".to_string(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(AudioPlugin { ..default() }),
        )
        .init_state::<GameState>()
        .add_plugins(crate::scenes::start_menu::MenuPlugin)
        .add_plugins(crate::scenes::world::PlatformerGamePlugin)
        .add_plugins(crate::player::PlatformerGamePlugin)
        .add_plugins(crate::physics::PlatformerGamePluginCollision)
        .add_plugins(audio::PlatformerGamePluginAudio)
        .run();
}
