use bevy::prelude::*;
use crate::game::position::Position2;

#[derive(Component)]
pub struct Player{
    pub pos: Position2,
    pub speed: f32,
}


impl Player{
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Player{
            pos : Position2::new(x, y),
            speed,
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.3, 0.0, 0.6),
            Vec2::new(50.0, 50.0),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::new(50.0, 50.0, 50.0),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut(){
        let mut movementx = 0.0;
        if keyboard_input.pressed(KeyCode::KeyD){
            movementx = time.delta_secs() 
        }

        if keyboard_input.pressed(KeyCode::KeyA){
            movementx = time.delta_secs() * -1.0;
        }

        player.pos.x += movementx * player.speed;
        transform.translation.x += movementx * player.speed;
    }
}