use crate::audio::{GameAudio, SoundType};
use crate::player::core::Player;
use crate::player::spawn::Collider;
use crate::scenes::menu::settings::{AudioSettingType, Settings};
use crate::scenes::world::coin::Coin;
use crate::scenes::world::spawn::{Tile, TileType};
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings};
use bevy::prelude::*;

pub const MAX_COLLISION_PUSH: f32 = 30.0;
pub const PLAYER_KNOCKBACK_MULT: f32 = 4.0;
pub const PUSH_SUBSTEPS: i32 = 10;

pub fn make_rect(transform: &Transform, collider: &Collider) -> Rect {
    Rect {
        min: transform.translation.truncate() - collider.half_size,
        max: transform.translation.truncate() + collider.half_size,
    }
}

pub fn claculate_collision(rect1: Rect, rect2: Rect) -> Vec2 {
    let center1 = (rect1.min + rect1.max) / 2.0;
    let center2 = (rect2.min + rect2.max) / 2.0;

    let size1 = rect1.max - rect1.min;
    let size2 = rect2.max - rect2.min;

    let distance = center2 - center1;
    let max_distance = (size1 + size2) / 2.0;

    let overlap_x = max_distance.x - distance.x.abs();
    let overlap_y = max_distance.y - distance.y.abs();

    if overlap_x <= 0.0 || overlap_y <= 0.0 {
        return Vec2::ZERO;
    }

    if overlap_x < overlap_y {
        Vec2::new(overlap_x * distance.x.signum(), 0.0)
    } else {
        Vec2::new(0.0, overlap_y * distance.y.signum())
    }
}

pub fn aabb_collision(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;

    let left_a = pos_a.x - half_a.x;
    let right_a = pos_a.x + half_a.x;
    let bottom_a = pos_a.y - half_a.y;
    let top_a = pos_a.y + half_a.y;

    let left_b = pos_b.x - half_b.x;
    let right_b = pos_b.x + half_b.x;
    let bottom_b = pos_b.y - half_b.y;
    let top_b = pos_b.y + half_b.y;

    !(left_a > right_b || right_a < left_b || top_a < bottom_b || bottom_a > top_b)
}

pub fn player_with_tile_collision_system(
    mut player_query: Query<(&mut Transform, &Sprite, &mut Player)>,
    tile_query: Query<(Entity, &Transform, &Sprite, &Tile), Without<Player>>,
) {
    for (mut player_transform, player_sprite, mut player) in &mut player_query {
        let mut player_pos = player_transform.translation.truncate();
        let player_size = player_sprite.custom_size.unwrap_or(Vec2::ZERO);

        for (tile_entity, tile_transform, _tile_sprite, tile) in &tile_query {
            let tile_size = tile.size;
            let tile_pos = tile_transform.translation.truncate();

            if !aabb_collision(player_pos, player_size, tile_pos, tile_size) {
                if player.collision == Some(tile_entity) {
                    player.jump = false;
                    player.collision = Option::<Entity>::default();
                }
                continue;
            }

            player.collision = Some(tile_entity);
            player.jump = true;

            match tile.kind {
                // Resolve walls only on X
                TileType::Wall => {
                    let delta = player_pos - tile_pos;
                    let combined_half = (player_size + tile_size) / 2.0;
                    let overlap_x = combined_half.x - delta.x.abs();

                    if overlap_x > 0.0 {
                        let push = overlap_x.min(MAX_COLLISION_PUSH);
                        if delta.x > 0.0 {
                            player_pos.x += push;
                        } else {
                            player_pos.x -= push;
                        }
                        player_transform.translation.x = player_pos.x;
                    }
                }

                // Resolve ground/platform only on Y.
                TileType::Ground | TileType::Platform => {
                    let delta = player_pos - tile_pos;
                    let combined_half = (player_size + tile_size) / 2.0;
                    let overlap_y = combined_half.y - delta.y.abs();

                    if overlap_y > 0.0 {
                        let push = overlap_y.min(MAX_COLLISION_PUSH);
                        if delta.y > 0.0 {
                            player_pos.y += push;
                        } else {
                            player.y_move = 0.0;
                            player_pos.y -= push;
                        }
                        player_transform.translation.y = player_pos.y;
                    }
                }
            }
        }
    }
}

fn intersects(a: &Rect, b: &Rect) -> bool {
    a.min.x < b.max.x && a.max.x > b.min.x && a.min.y < b.max.y && a.max.y > b.min.y
}

pub fn player_with_player(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Collider, &mut Player)>,
    audio_assets: Res<GameAudio>,
    settings: Res<Settings>,
) {
    let mut pushes: Vec<(Entity, f32, f32)> = Vec::new();
    let mut combinations = query.iter_combinations_mut();

    while let Some([player1, player2]) = combinations.fetch_next() {
        let (e1, t1, c1, mut p1) = player1;
        let (e2, t2, c2, mut p2) = player2;

        let r1 = make_rect(&t1, c1);
        let r2 = make_rect(&t2, c2);

        if !intersects(&r1, &r2) {
            continue;
        }

        let details = claculate_collision(r1, r2);

        // Side collision (X): strong symmetric knockback
        if details.x != 0.0 {
            commands.spawn((
                AudioPlayer::new(audio_assets.fight.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::Linear(settings.hit_volume),
                    ..PlaybackSettings::ONCE
                },
                SoundType(AudioSettingType::Damage),
            ));

            p1.points += settings.hit_score;
            p2.points += settings.hit_score;

            let sep = details.x.abs().min(MAX_COLLISION_PUSH) / 2.0;
            let kb = (p1.collision_reaction_x * PLAYER_KNOCKBACK_MULT)
                .min(MAX_COLLISION_PUSH * PLAYER_KNOCKBACK_MULT);

            let dir = details.x.signum();
            let push1 = -(sep + kb) * dir;
            let push2 = (sep + kb) * dir;

            pushes.push((e1, push1, 0.0));
            pushes.push((e2, push2, 0.0));
        }
        // Vertical collision (Y): top player bounces up, bottom stays mostly stable
        else if details.y != 0.0 {
            commands.spawn((
                AudioPlayer::new(audio_assets.damage.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::Linear(settings.hit_volume),
                    ..PlaybackSettings::ONCE
                },
                SoundType(AudioSettingType::Damage),
            ));

            let sep = details.y.abs().min(MAX_COLLISION_PUSH);
            let kb = (p1.collision_reaction_y * PLAYER_KNOCKBACK_MULT)
                .min(MAX_COLLISION_PUSH * PLAYER_KNOCKBACK_MULT);

            // details.y > 0 => p2 is above p1
            if details.y > 0.0 {
                pushes.push((e2, 0.0, sep + kb));
            } else {
                pushes.push((e1, 0.0, sep + kb));
            }

            p1.y_move = 0.0;
            p2.y_move = 0.0;

            if p1.jump && !p2.jump {
                p2.points += settings.damage_score;
            } else if p2.jump && !p1.jump {
                p1.points += settings.damage_score;
            }
        }
    }

    // Apply pushes in small steps to reduce tunneling through tiles (not working)
    for (entity, x, y) in pushes {
        if let Ok((_e, mut transform, _collider, _player)) = query.get_mut(entity) {
            let steps_i32 = PUSH_SUBSTEPS.max(1);
            let steps_f32 = steps_i32 as f32;

            let sx = x / steps_f32;
            let sy = y / steps_f32;

            for _ in 0..steps_i32 {
                transform.translation.x += sx;
                transform.translation.y += sy;
            }
        }
    }
}

pub fn player_with_coin_collision_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player, &Sprite)>,
    coin_query: Query<(Entity, &Transform, &Sprite, &Coin)>,
    audio_assets: Res<GameAudio>,
    settings: Res<Settings>,
) {
    for (player_transform, mut player, player_sprite) in &mut player_query {
        let player_pos = player_transform.translation.truncate();
        let player_size = player_sprite.custom_size.unwrap_or(Vec2::ZERO);

        for (coin_entity, coin_transform, coin_sprite, _coin) in &coin_query {
            let coin_size = coin_sprite.custom_size.unwrap_or(Vec2::ZERO);
            let coin_pos = coin_transform.translation.truncate();

            if aabb_collision(player_pos, player_size, coin_pos, coin_size) {
                commands.spawn((
                    AudioPlayer::new(audio_assets.coin_pickup.clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: bevy::audio::Volume::Linear(settings.coin_volume),
                        ..PlaybackSettings::ONCE
                    },
                    SoundType(AudioSettingType::Coin),
                ));

                player.points += settings.coin_score;
                commands.entity(coin_entity).try_despawn();
            }
        }
    }
}
