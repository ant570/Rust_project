use bevy::prelude::*;
use crate::game::position::Position2;

#[derive(Component)]
pub struct Player{
    pub pos: Position2,
    pub speed_x: f32,
    pub jump_speed: f32,
    pub gravity: f32,
}



impl Player{
    pub fn new(x: f32, y: f32, speed_x: f32, jump_speed: f32, gravity: f32) -> Self {
        Player{
            pos : Position2::new(x, y),
            speed_x,
            jump_speed,
            gravity,
        }
    }
}

pub fn spawn_player(mut commands: Commands,  asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let texture_handle: Handle<Image> = asset_server.load("mario1.jpg");
    commands.spawn((
        // 1. Dwa komponenty transformacji
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(), // KLUCZOWY komponent do renderowania
        
        // 2. Komponent wizualny
        Sprite {
            image: texture_handle, 
            ..default()
        },
        // 3. Komponent gracza
        Player::new(50.0, 50.0, 50.0, 100.0, 1.0),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut(){
        let mut movement_x = 0.0;
        let mut movement_y = 0.0;

        //Movement x
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            movement_x = time.delta_secs() 
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft){
            movement_x = time.delta_secs() * -1.0;
        }
        player.pos.x += movement_x * player.speed_x;
        transform.translation.x += movement_x * player.speed_x;

        //Movement y
        if keyboard_input.pressed(KeyCode::Space) ||
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp){
            movement_y = time.delta_secs() * player.jump_speed;
        }
        movement_y -= player.gravity;
        player.pos.y += movement_y;
        transform.translation.y += movement_y;
        
    }
}