use bevy::prelude::*;
pub struct PlatformerGamePlugin;

mod spawn;
mod platforms;

impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn::spawn_camera, spawn::spawn_map))
            .add_systems(Update, platforms::move_platforms_system);
    }
}
