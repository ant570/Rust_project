use bevy::prelude::*;

pub mod world;

pub struct PlatformerGamePlugin;


impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (world::spawn_camera, world::spawn_map));
    }
}
