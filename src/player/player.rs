use bevy::prelude::*;
use crate::player::position::Position2;

pub enum Control{
    Wasd,
    Arrows
}

#[derive(Component)]
pub struct Player{
    pub pos: Position2,
    pub speed_x: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub control: Control,
    pub movement: bool,
    pub collision_reaction_x: f32,
    pub collision_reaction_y: f32,
    pub jump: bool,
    pub collision: Option<Entity>,
    pub y_move: f32,
    pub jump_height: f32,
    pub points: u32,
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
            points: 0,
        }
    }

}