use bevy::prelude::*;
use crate::GameState;
 use crate::scenes::start_menu::start_menu::menu_action;

pub mod start_menu;
pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(crate::scenes::game_state::GameState::StartMenu), start_menu::start_menu)
        .add_systems(
                Update, 
                menu_action.run_if(in_state(GameState::StartMenu))
            )
        .add_systems(OnExit(crate::scenes::game_state::GameState::StartMenu), start_menu::cleanup_menu);
    }
}