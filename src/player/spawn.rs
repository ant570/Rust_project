use std::os::linux::fs;

use crate::audio::GameAudio;
use crate::player::attack::{AttackState, STICK_BASE_RANGE, STICK_THICKNESS, Stick};
use crate::player::player::Control;
use crate::player::player::Control::Arrows;
use crate::player::player::Control::Wasd;
use crate::player::player::Player;
use bevy::prelude::*;
use crate::audio::SoundType;
use crate::scenes::menu::settings::AudioSettingType;
use crate::scenes::menu::settings::Settings;
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings};
const PLAYER_WIDTH: f32 = 150.0;
const PLAYER_HEIGHT: f32 = 150.0;
#[derive(Component)]
pub struct Collider {
    pub half_size: Vec2,
}

pub fn spawn_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_player(
        &mut commands,
        asset_server.load("players/player1.png"),
        200.0,
        0.0,
        Arrows,
        true,
        200.0,
        200.0,
    );

    spawn_player(
        &mut commands,
        asset_server.load("players/player2.png"),
        -200.0,
        0.0,
        Wasd,
        true,
        200.0,
        200.0,
    );
}

pub fn spawn_player(
    commands: &mut Commands,
    player_texture: Handle<Image>,
    x: f32,
    y: f32,
    control: Control,
    movement: bool,
    collision_reaction_x: f32,
    collision_reaction_y: f32,
) {
    let player_size = Vec2::new(PLAYER_HEIGHT, PLAYER_HEIGHT);
    let mut _entity = commands
        .spawn((
            Sprite {
                image: player_texture,
                custom_size: Some(player_size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Player::new(
                x,
                y,
                500.0,
                3000.0,
                13.0,
                control,
                movement,
                collision_reaction_x,
                collision_reaction_y,
                false,
                700.0,
            ),
            AttackState {
                charge: 0.0,
                facing: 1.0,
                was_pressed: false,
                just_released: false,
                release_charge: 0.0,
                swing_timer: 0.0,
            },
            Collider {
                half_size: Vec2::new(PLAYER_HEIGHT / 2.0, PLAYER_WIDTH / 2.0),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    color: Color::srgb(0.55, 0.27, 0.07),
                    custom_size: Some(Vec2::new(STICK_BASE_RANGE, STICK_THICKNESS)),
                    ..default()
                },
                Transform::from_xyz(STICK_BASE_RANGE / 2.0, 0.0, 1.0),
                Stick,
            ));
            parent.spawn((
                Text2d::new("0"),
                TextFont {
                    font_size: 70.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_xyz(10.0, PLAYER_HEIGHT / 2.0 + 50.0, 0.0),
                ScoreText,
            ));
            parent.spawn((
                Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(0.0, 10.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, PLAYER_HEIGHT / 2.0 + 10.0, 0.1),
                ChargeBar,
            ));
        });
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ChargeBar;

pub fn check_player_fall(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Player)>,
    audio_assets: Res<GameAudio>,
    settings: Res<Settings>
) {
    let mut fallen_entities = Vec::new();
    let fall_limit = -crate::scenes::world::utils::WORLD_HEIGHT / 2.0 - 200.0;

    for (entity, transform, _) in query.iter() {
        if transform.translation.y < fall_limit {
            fallen_entities.push(entity);
        }
    }

    if fallen_entities.is_empty() {
        return;
    }

    for (entity, mut transform, mut player) in query.iter_mut() {
        if fallen_entities.contains(&entity) {
            commands.spawn((
                AudioPlayer::new(audio_assets.lose.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::Linear(settings.hit_volume),
                    ..PlaybackSettings::ONCE
                },
                SoundType(AudioSettingType::Fail),
            ));

            transform.translation.y = 200.0;
            transform.translation.x = 0.0;

            let respawn_x = match player.control {
                Control::Arrows => 200.0,
                Control::Wasd => -200.0,
            };
            transform.translation.x = respawn_x;

            player.pos.x = respawn_x;
            player.pos.y = 200.0;
            player.y_move = 0.0;

            player.jump = false;
            player.y_move = 0.0;
        } else {
            player.points += settings.fall_score;
        }
    }
}
