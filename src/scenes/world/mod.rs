use bevy::prelude::*;
use crate::scenes::world::platforms_spawner::PlatformSpawnTimer;
use crate::scenes::game_state::GameState;
pub struct PlatformerGamePlugin;

pub mod coin;
pub mod platforms;
mod platforms_spawner;
pub mod spawn;
pub mod utils;

impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlatformSpawnTimer>()
            .add_systems(Startup, spawn::spawn_camera)
            .add_systems(
                OnTransition { 
                    exited: GameState::StartMenu, 
                    entered: GameState::Playing
                },
                (
                    spawn::spawn_map,
                    platforms_spawner::setup_platform_spawner
                )
            )
            .add_systems(
                Update,
                (
                    platforms::move_platforms_system,
                    platforms_spawner::platform_spawner_system,
                    coin::animate_coins
                )
                .run_if(in_state(crate::scenes::game_state::GameState::Playing))
            );
    }
}
