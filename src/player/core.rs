use crate::player::position::Position2;
use bevy::prelude::*;

//Typy sterowania
pub enum Control {
    Wasd,
    Arrows,
}

pub struct PlayerConfig {
    pub x: f32,
    pub y: f32,
    pub speed_x: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub control: Control,
    pub movement: bool,
    pub collision_reaction_x: f32,
    pub collision_reaction_y: f32,
    pub jump: bool,
    pub jump_height: f32,
}

#[derive(Component)]
pub struct Player {
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

impl Player {
    pub fn new(cfg: PlayerConfig) -> Self {
        Player {
            pos: Position2::new(cfg.x, cfg.y),
            speed_x: cfg.speed_x,
            jump_speed: cfg.jump_speed,
            gravity: cfg.gravity,
            control: cfg.control,
            movement: cfg.movement,
            collision_reaction_x: cfg.collision_reaction_x,
            collision_reaction_y: cfg.collision_reaction_y,
            jump: cfg.jump,
            collision: Option::<Entity>::default(),
            y_move: 0.0,
            jump_height: cfg.jump_height,
            points: 0,
        }
    }
}
