use bevy::prelude::*;
pub struct PlatformerGamePlugin;

pub mod spawn;
pub mod platforms;
mod platforms_spawner;
mod utils;
mod coin;

impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn::spawn_camera, spawn::spawn_map))
            .add_systems(Update, platforms::move_platforms_system)
            //.add_systems(Startup, spawn::spawn_initial_platforms);
            .add_systems(Startup, platforms_spawner::setup_platform_spawner)
            .add_systems(Update, platforms_spawner::platform_spawner_system)
            .add_systems(Update, coin::animate_coins);
    }
}
