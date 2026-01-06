use bevy::prelude::*;
use crate::player::player::Player;
use std::process;
use bevy::input::ButtonInput;
use bevy::time::Time;
use crate::player::player::Control;


pub fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
){
    //Zakończenie gry 
    if keyboard_input.just_pressed(KeyCode::Escape){
        process::exit(0);
    }
    else if keyboard_input.just_pressed(KeyCode::Space) {
        //Zapauzowanie gry
        for (_transform, mut player) in query.iter_mut(){
            if player.movement == true {
                player.movement = false;
            }
            else{
                player.movement = true;
            }
        }
    }
    else{
        //Ruch gracza
        player_movement(keyboard_input, query, time);
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut(){

        //Sprawdzenie czy nie jest zatrzymany
        if !player.movement{
            continue;
        }

        let mut movement_x = 0.0;
        let mut movement_y = 0.0;

        //Ruch gracza w zalezności od typu sterowania.
        match player.control {
            Control::Wasd => {
                //Movement x
                if keyboard_input.pressed(KeyCode::KeyD) {
                    movement_x = time.delta_secs() ;
                }
                if keyboard_input.pressed(KeyCode::KeyA){
                    movement_x = time.delta_secs() * -1.0;
                }

                //Movement y
                if keyboard_input.just_pressed(KeyCode::KeyW) && player.jump {
                    player.y_move += player.jump_height;
                    player.jump = false;
                }
            }
            Control::Arrows => {
                if !player.movement{
                    continue;
                }
                //Movement x
                if keyboard_input.pressed(KeyCode::ArrowRight) {
                    movement_x = time.delta_secs() ;
                }
                if keyboard_input.pressed(KeyCode::ArrowLeft){
                    movement_x = time.delta_secs() * -1.0;
                }

                
                //Movement y
                if keyboard_input.just_pressed(KeyCode::ArrowUp) && player.jump{
                    println!("{}", player.jump_speed);
                    player.y_move += player.jump_height;
                    player.jump = false;
                }
            }
        }
        if player.y_move > 0.0{
            movement_y = f32::min(player.y_move, time.delta_secs() * player.jump_speed);
            player.y_move -= movement_y;
        }

        player.pos.x += movement_x * player.speed_x;
        transform.translation.x += movement_x * player.speed_x;
        movement_y -= player.gravity;
        player.pos.y += movement_y; //grawitacja
        transform.translation.y += movement_y;
        
    }
}