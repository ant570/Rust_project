use bevy::prelude::*;
mod collisions;
pub struct PlatformerGamePluginCollision;

impl Plugin for PlatformerGamePluginCollision {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, collisions::player_with_tile_collision_system);
    }
}

