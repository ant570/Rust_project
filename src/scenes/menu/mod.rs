use bevy::prelude::*;
use crate::GameState;
use crate::scenes::menu::finish::spawn_finish_menu;
use crate::scenes::menu::start_menu::menu_action;
use crate::scenes::menu::how_to_play::spawn_htp;
use crate::scenes::menu::settings::spawn_settings;

pub mod start_menu;
pub mod how_to_play;
pub mod pause_menu;
pub mod finish;
pub mod settings;

pub struct MenuPlugin;
#[derive(Component)]
pub struct OnMenuScreen;
pub fn cleanup_menu(
    mut commands: Commands, 
    query: Query<Entity, With<OnMenuScreen>>
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}



impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(crate::scenes::game_state::GameState::StartMenu), start_menu::start_menu)
        .add_systems(
                Update, 
                menu_action.run_if(in_state(GameState::StartMenu))
            )
        .add_systems(OnExit(crate::scenes::game_state::GameState::StartMenu), cleanup_menu)
        .add_systems(OnEnter(GameState::HowToPlay1), spawn_htp)
        .add_systems(Update, how_to_play::htp_action.run_if(in_state(GameState::HowToPlay1)))
        .add_systems(OnExit(GameState::HowToPlay1), cleanup_menu)
        .add_systems(OnEnter(GameState::Paused), pause_menu::pause_menu)
        .add_systems(Update, pause_menu::pause_menu_action.run_if(in_state(GameState::Paused)))
        .add_systems(OnExit(GameState::Paused), cleanup_menu)
        .add_systems(OnEnter(GameState::HowToPlay2), spawn_htp)
        .add_systems(Update, how_to_play::htp_action.run_if(in_state(GameState::HowToPlay2)))
        .add_systems(OnExit(GameState::HowToPlay2), cleanup_menu)
        .add_systems(OnEnter(GameState::Finished), spawn_finish_menu)
        .add_systems(Update, finish::finish_menu_action.run_if(in_state(GameState::Finished)))
        .add_systems(OnExit(GameState::Finished), cleanup_menu)
        .add_systems(OnEnter(GameState::SettingsStart), spawn_settings)
        .add_systems(Update, settings::settings_action.run_if(in_state(GameState::SettingsStart)))
        .add_systems(OnExit(GameState::SettingsStart), cleanup_menu)
        .add_systems(OnEnter(GameState::SettingsPause), spawn_settings)
        .add_systems(Update, settings::settings_action.run_if(in_state(GameState::SettingsPause)))
        .add_systems(OnExit(GameState::SettingsPause), cleanup_menu)
        .init_resource::<settings::AudioSettings>()
        .add_systems(Update, settings::settings_action.run_if(
            in_state(GameState::SettingsStart)
            .or(in_state(GameState::SettingsPause))
        ));

    }
}