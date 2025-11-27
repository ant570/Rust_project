use bevy::prelude::*;
mod world;
use world::spawn_map;
pub mod position;
pub mod player;

pub struct PlatformerGamePlugin;

impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}
