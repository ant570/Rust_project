use crate::audio::GameAudio;
use crate::audio::SoundType;
use crate::player::player::Control;
use crate::player::player::Player;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use crate::scenes::menu::settings::Settings;
use bevy::time::Time;
use crate::scenes::menu::settings::AudioSettingType;
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings};

pub fn keyboard_input(
    commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    audio_assets: Res<GameAudio>,
    mut next_state: ResMut<NextState<crate::scenes::game_state::GameState>>,
    settings: Res<Settings>,
) {
    //Zakończenie gry
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(crate::scenes::game_state::GameState::Paused);
    } 
    else {
        //Ruch gracza
        player_movement(commands, keyboard_input, query, time, audio_assets, settings);
    }
}

pub fn player_movement(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    audio_assets: Res<GameAudio>,
    settings: Res<Settings>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        //Sprawdzenie czy nie jest zatrzymany
        if !player.movement {
            continue;
        }

        let mut movement_x = 0.0;
        let mut movement_y = 0.0;

        //Ruch gracza w zalezności od typu sterowania.
        match player.control {
            Control::Wasd => {
                //Movement x
                if keyboard_input.pressed(KeyCode::KeyD) {
                    movement_x = time.delta_secs();
                }
                if keyboard_input.pressed(KeyCode::KeyA) {
                    movement_x = -time.delta_secs();
                }

                //Movement y
                if keyboard_input.just_pressed(KeyCode::KeyW) && player.jump {
                    commands.spawn((
                        AudioPlayer::new(audio_assets.jump.clone()),
                        PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: bevy::audio::Volume::Linear(settings.jump_volume),
                            ..PlaybackSettings::ONCE
                        },
                        SoundType(AudioSettingType::Jump),
                    ));
                    player.y_move += player.jump_height;
                    player.jump = false;
                }
            }
            Control::Arrows => {
                if !player.movement {
                    continue;
                }
                //Movement x
                if keyboard_input.pressed(KeyCode::ArrowRight) {
                    movement_x = time.delta_secs();
                }
                if keyboard_input.pressed(KeyCode::ArrowLeft) {
                    movement_x = -time.delta_secs();
                }

                //Movement y
                if keyboard_input.just_pressed(KeyCode::ArrowUp) && player.jump {
                    commands.spawn((
                        AudioPlayer::new(audio_assets.jump.clone()),
                        PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: bevy::audio::Volume::Linear(settings.jump_volume),
                            ..PlaybackSettings::ONCE
                        },
                        SoundType(AudioSettingType::Jump),
                    ));
                    println!("{}", player.jump_speed);
                    player.y_move += player.jump_height;
                    player.jump = false;
                }
            }
        }
        if player.y_move > 0.0 {
            movement_y = f32::min(player.y_move, time.delta_secs() * player.jump_speed);
            player.y_move -= movement_y;
        }

        player.pos.x += movement_x * player.speed_x;
        transform.translation.x += movement_x * player.speed_x;
        movement_y -= player.gravity;
        transform.translation.y += movement_y;
        player.pos.y += movement_y; //grawitacja
        
    }
}
