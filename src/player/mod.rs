use bevy::prelude::*;
use crate::GameState;

use crate::in_state;
pub struct PlatformerGamePlugin;
pub mod attack;
pub mod movement;
pub mod player;
pub mod position;
pub mod score;
pub mod spawn;

impl Plugin for PlatformerGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnTransition { 
                    exited: GameState::StartMenu, 
                    entered: GameState::Playing
                }, 
                spawn::spawn_players
            )
            .add_systems(
                Update,
                (
                    movement::keyboard_input,
                    attack::stick_attack,
                    spawn::check_player_fall,
                    score::update_score_text,
                )
                .run_if(in_state(GameState::Playing)),
            );
    }
}
