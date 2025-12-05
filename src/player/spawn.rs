use bevy::prelude::*;
use crate::player::position::Position2;
use bevy::window::PrimaryWindow;
use crate::player::spawn::Control::Wasd;

const PLAYER_WIDTH: f32 = 200.0;
const PLAYER_HEIGHT: f32 = 200.0;

#[derive(Component)]
pub struct Player{
    pub pos: Position2,
    pub speed_x: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub control: Control,
}


pub enum Control{
    Wasd,
    Arrows2
}

impl Player{
    pub fn new(x: f32, y: f32, speed_x: f32, jump_speed: f32, gravity: f32, control: Control) -> Self {
        Player{
            pos : Position2::new(x, y),
            speed_x,
            jump_speed,
            gravity,
            control
        }
    }
}

pub fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    spawn_player(
        &mut commands,
        asset_server.load("players/player1.png"),
        200.0, 0.0,
        Wasd
    );

    spawn_player(
        &mut commands,
        asset_server.load("players/player2.png"),
        -200.0, 0.0,
        Wasd,
    );


}

pub fn spawn_player(
    commands: &mut Commands,
    player_texture : Handle<Image>,
    x : f32,
    y : f32,
    control: Control,
) {
    let player_size = Vec2::new(PLAYER_HEIGHT, PLAYER_HEIGHT);
    let mut entity = commands.spawn((
        Sprite{
            custom_size: Some(player_size),
            image: player_texture,
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        Player::new(x, y, 100.0, 150.0, 1.0, control),
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

        match player.control {
            Control::Wasd => {
                //Movement x
                if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
                    movement_x = time.delta_secs() ;
                }
                if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft){
                    movement_x = time.delta_secs() * -1.0;
                }

                //Movement y
                if keyboard_input.pressed(KeyCode::Space) ||
                    keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp){
                    movement_y = time.delta_secs() * player.jump_speed;
                }
            }
            Control::Arrows2 => todo!()
        }

        player.pos.x += movement_x * player.speed_x;
        transform.translation.x += movement_x * player.speed_x;
        movement_y -= player.gravity;
        player.pos.y += movement_y;
        transform.translation.y += movement_y;
        
    }
}