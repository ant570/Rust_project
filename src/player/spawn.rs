use bevy::prelude::*;
use crate::player::player::Control::Wasd;
use crate::player::player::Control::Arrows;
use crate::player::player::Control;
use crate::player::player::Player;

const PLAYER_WIDTH: f32 = 200.0;
const PLAYER_HEIGHT: f32 = 200.0;
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
    let mut _entity = commands.spawn((
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

