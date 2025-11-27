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
            Vec2::new(800.0, 50.0),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::new(800.0, 50.0, 20.0),
    ));
}czy