use bevy::prelude::*;
use crate::player::position::Position2;
use bevy::window::PrimaryWindow;
use crate::player::spawn::Control::Wasd;
use crate::player::spawn::Control::Arrows;
use bevy::app::AppExit;
use std::process;
use crate::world::platforms::PlatformMover;
use std::ptr::null;

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
    collision_reaction_x: f32,
    collision_reaction_y: f32,
    pub jump: bool,
    pub collision: Option<Entity>,
    pub y_move: f32,
    pub jump_height: f32,
}


pub enum Control{
    Wasd,
    Arrows
}

impl Player{
    pub fn new(x: f32,
        y: f32, 
        speed_x: f32, 
        jump_speed: f32,
        gravity: f32, 
        control: Control,
        movement: bool,
        collision_reaction_x: f32,
        collision_reaction_y: f32,
        jump: bool,
        jump_height: f32,

    ) -> Self 
    {
        Player{
            pos : Position2::new(x, y),
            speed_x,
            jump_speed,
            gravity,
            control,
            movement,
            collision_reaction_x,
            collision_reaction_y,
            jump,
            collision: Option::<Entity>::default(),
            y_move: 0.0,
            jump_height,
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
        200.0, 200.0,
    );

    spawn_player(
        &mut commands,
        asset_server.load("players/player2.png"),
        -200.0, 0.0,
        Wasd,
        true,
        200.0, 200.0,
    );


}

pub fn spawn_player(
    commands: &mut Commands,
    player_texture : Handle<Image>,
    x : f32,
    y : f32,
    control: Control,
    movement: bool,
    collision_reaction_x: f32,
    collision_reaction_y: f32,
) {
    let player_size = Vec2::new(PLAYER_HEIGHT, PLAYER_HEIGHT);
    let mut entity = commands.spawn((
        Sprite{
            custom_size: Some(player_size),
            image: player_texture,
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        Player::new(x, y, 500.0, 3000.0, 13.0, control, movement, collision_reaction_x, collision_reaction_y, false, 700.0),
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

pub fn claculate_collision(rect1: Rect, rect2: Rect) -> Vec2{
    let center1 = (rect1.min + rect1.max) / 2.0;
    let center2 = (rect2.min + rect2.max) / 2.0;

    let size1= rect1.max - rect1.min;
    let size2 = rect2.max - rect2.min;

    let distance= center2 - center1;;
    let max_distance = (size1 + size2) / 2.0;

    let overlap_x = max_distance.x - distance.x.abs();
    let overlap_y = max_distance.y - distance.y.abs();

    if overlap_x <= 0.0 || overlap_y <= 0.0 {
        return Vec2::new(0.0, 0.0); 
    }
    
    if overlap_x < overlap_y {
        // x collision
        Vec2::new(distance.x, 0.0)
    } else {
        // y collision
        Vec2::new(0.0, distance.y)
    }
}
pub fn players_collsions(
    mut query: Query<(Entity, &mut Transform, &Collider, &Player)>
){
    let mut data_vector: Vec<(Entity, f32, f32)> = Vec::new();
    query.iter_combinations().for_each(|[player1, player2]| {
        let (entity_id1, transform1, collider1, player1_component) = player1;
        let (entity_id2, transform2, collider2, player2_component) = player2;
        
        let rect1 =  make_rect(transform1, collider1);
        let rect2 = make_rect(transform2, collider2);

        if intersects(&rect1, &rect2) {
            
            let details = claculate_collision(rect1, rect2);
            if details[0] != 0.0{
                //x collisions
                let distance = details[0];
                if distance > 0.0{
                    data_vector.push((
                        entity_id1,
                        player1_component.collision_reaction_x * -1.0,
                        0.0,
                    ));

                    data_vector.push((
                        entity_id2,
                        player1_component.collision_reaction_x,
                        0.0,
                    ));
                }
                else if distance < 0.0 {
                    let distance = details[1];
                    data_vector.push((
                        entity_id1,
                        player1_component.collision_reaction_x,
                        0.0,
                    ));

                    data_vector.push((
                        entity_id2,
                        player1_component.collision_reaction_x * -1.0,
                        0.0,
                    ));
                }
            }
            else if details[1] > 0.0{
                //y collisions
                let distance = details[1];
                if distance > 0.0{
                    data_vector.push((
                        entity_id2,
                        0.0,
                        player1_component.collision_reaction_y,
                    ));
                }
                else{
                    data_vector.push((
                        entity_id1,
                        0.0,
                        player1_component.collision_reaction_y,
                    ));
                }
            }
        }

        

    }
    );
    for (entity, x, y) in data_vector.iter_mut(){
        match query.get_mut(*entity){
            Ok((entity_pom, mut transform, collider, player_comp)) => {
                // Zmień pozycję gracza
                transform.translation.x += *x;
                transform.translation.y += *y; 
            }
            _ => {}
        }
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