use bevy::prelude::*;
use crate::player::position::Position2;
use bevy::window::PrimaryWindow;
use crate::player::spawn::Control::Wasd;
use crate::player::spawn::Control::Arrows;
use bevy::app::AppExit;
use std::process;

const PLAYER_WIDTH: f32 = 200.0;
const PLAYER_HEIGHT: f32 = 200.0;

#[derive(Component)]
pub struct Player{
    pub pos: Position2,
    pub speed_x: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub control: Control,
    pub movement: bool,
}


pub enum Control{
    Wasd,
    Arrows
}

impl Player{
    pub fn new(x: f32, y: f32, speed_x: f32, jump_speed: f32, gravity: f32, control: Control, movement: bool) -> Self {
        Player{
            pos : Position2::new(x, y),
            speed_x,
            jump_speed,
            gravity,
            control,
            movement
        }
    }
}

#[derive(Component)]
pub struct Collider{
    pub half_size: Vec2,
}

pub fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    spawn_player(
        &mut commands,
        asset_server.load("players/player1.png"),
        200.0, 0.0,
        Arrows,
        true,
    );

    spawn_player(
        &mut commands,
        asset_server.load("players/player2.png"),
        -200.0, 0.0,
        Wasd,
        true,
    );


}

pub fn spawn_player(
    commands: &mut Commands,
    player_texture : Handle<Image>,
    x : f32,
    y : f32,
    control: Control,
    movement: bool,
) {
    let player_size = Vec2::new(PLAYER_HEIGHT, PLAYER_HEIGHT);
    let mut entity = commands.spawn((
        Sprite{
            custom_size: Some(player_size),
            image: player_texture,
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        Player::new(x, y, 100.0, 150.0, 1.0, control, movement),
        Collider{
            half_size: Vec2::new(PLAYER_HEIGHT / 2.0, PLAYER_WIDTH / 2.0),
        },
    
    ));

    
}

pub fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    mut app_exit_events: MessageWriter<AppExit>,
){
    //Zakończenie gry 
    if keyboard_input.just_pressed(KeyCode::Escape){
        process::exit(0);
    }
    else if keyboard_input.just_pressed(KeyCode::Space) {
        //Zapauzowanie gry
        for (mut transform, mut player) in query.iter_mut(){
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
pub fn make_rect(
    transform: &Transform,
    collider: &Collider,
) -> Rect {
    Rect{
        //left_bottom point
        min: transform.translation.truncate() - collider.half_size,
        //right_up point
        max: transform.translation.truncate() + collider.half_size
    }
}

fn intersects(a: &Rect, b: &Rect) -> bool {
    a.min.x < b.max.x && a.max.x > b.min.x &&
    a.min.y < b.max.y && a.max.y > b.min.y
}

pub fn players_collsions(
    mut query: Query<(&Transform, &Collider, &Player)>
){
    query.iter_combinations().for_each(|[player1, player2]| {
        let (transform1, collider1, _) = player1;
        let (transform2, collider2, _) = player2;
        
        let rect1 =  make_rect(transform1, collider1);
        let rect2 = make_rect(transform2, collider2);

        if intersects(&rect1, &rect2) {
            process::exit(0);
        }

    }
    );
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
                if keyboard_input.pressed(KeyCode::KeyW) {
                    movement_y = time.delta_secs() * player.jump_speed;
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
                if keyboard_input.pressed(KeyCode::ArrowUp){
                    movement_y = time.delta_secs() * player.jump_speed;
                }
            }
        }

        player.pos.x += movement_x * player.speed_x;
        transform.translation.x += movement_x * player.speed_x;
        movement_y -= player.gravity;
        player.pos.y += movement_y; //grawitacja
        transform.translation.y += movement_y;
        
    }
}