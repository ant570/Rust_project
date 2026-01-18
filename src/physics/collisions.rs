use crate::audio::{GameAudio, SoundType};
use crate::player::player::Player;
use crate::player::spawn::Collider;
use crate::scenes::world::coin::Coin;
use crate::scenes::world::spawn::{Tile, TileType};
use bevy::prelude::*;
use crate::scenes::menu::settings::AudioSettingType;
use crate::scenes::menu::settings::AudioSettings;
use bevy::audio::{AudioPlayer, PlaybackMode, PlaybackSettings};
pub const MAX_COLLISION_PUSH: f32 = 30.0;

pub fn make_rect(transform: &Transform, collider: &Collider) -> Rect {
    Rect {
        //left_bottom point
        min: transform.translation.truncate() - collider.half_size,
        //right_up point
        max: transform.translation.truncate() + collider.half_size,
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

    !(left_a > right_b
        || right_a < left_b
        || top_a < bottom_b     // Sprawdzeenie czy jest kolizja miedzy a i b, czy sie na siebie nachodza
        || bottom_a > top_b)
}

pub fn player_with_tile_collision_system(
    mut player_query: Query<(&mut Transform, &Sprite, &mut Player)>,
    tile_query: Query<(Entity, &Transform, &Sprite, &Tile), Without<Player>>,
) {
    for (mut player_transform, player_sprite, mut player) in &mut player_query {
        let mut player_pos = player_transform.translation.truncate(); // bez pozycji z osi z
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
                TileType::Ground | TileType::Wall | TileType::Platform => {
                    let delta = player_pos - tile_pos;

                    let combined_half = (player_size + tile_size) / 2.0;
                    let mut overlap_x = combined_half.x - delta.x.abs();
                    let mut overlap_y = combined_half.y - delta.y.abs(); // sprawdzenie nakladania sie
                    overlap_x = overlap_x.max(-MAX_COLLISION_PUSH);
                    overlap_y = overlap_y.max(-MAX_COLLISION_PUSH);
                    overlap_x = overlap_x.min(MAX_COLLISION_PUSH);
                    overlap_y = overlap_y.min(MAX_COLLISION_PUSH);
                    if overlap_x < overlap_y {
                        if delta.x > 0.0 {
                            player_pos.x += overlap_x;
                        } else {
                            player_pos.x -= overlap_x;
                        }
                    } else if delta.y > 0.0 {
                        player_pos.y += overlap_y;
                    } else {
                        player.y_move = 0.0;
                        player_pos.y -= overlap_y;
                    }
                    player_transform.translation.x = player_pos.x;
                    player_transform.translation.y = player_pos.y;
                }
            }
        }
    }
}

fn intersects(a: &Rect, b: &Rect) -> bool {
    a.min.x < b.max.x && a.max.x > b.min.x && a.min.y < b.max.y && a.max.y > b.min.y
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

pub fn player_with_player(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Collider, &mut Player)>,
    audio_assets: Res<GameAudio>,
    settings: Res<AudioSettings>,
) {
    let mut data_vector: Vec<(Entity, f32, f32)> = Vec::new();
    let mut combinations = query.iter_combinations_mut();

    // 2. Używamy pętli while, aby bezpiecznie pobierać kolejne pary
    while let Some([player1, player2]) = combinations.fetch_next() {
        let (entity_id1, transform1, collider1, mut player1_component) = player1;
        let (entity_id2, transform2, collider2, mut player2_component) = player2;

        let rect1 = make_rect(&transform1, collider1);
        let rect2 = make_rect(&transform2, collider2);

        if intersects(&rect1, &rect2) {
            let details = claculate_collision(rect1, rect2);
            if details[0] != 0.0 {
                commands.spawn((
                    AudioPlayer::new(audio_assets.fight.clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: bevy::audio::Volume::Linear(settings.hit_volume),
                        ..PlaybackSettings::ONCE
                    },
                    SoundType(AudioSettingType::Damage),
                ));
                player1_component.points += 1;
                player2_component.points += 1;
                //x collisions
                let distance = details[0];
                if distance > 0.0 {
                    data_vector.push((entity_id1, -player1_component.collision_reaction_x, 0.0));

                    data_vector.push((entity_id2, player1_component.collision_reaction_x, 0.0));
                } else if distance < 0.0 {
                    data_vector.push((entity_id1, player1_component.collision_reaction_x, 0.0));

                    data_vector.push((entity_id2, -player1_component.collision_reaction_x, 0.0));
                }
            } else if details[1] != 0.0 {
                commands.spawn((
                    AudioPlayer::new(audio_assets.damage.clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: bevy::audio::Volume::Linear(settings.hit_volume),
                        ..PlaybackSettings::ONCE
                    },
                    
                    SoundType(AudioSettingType::Damage),
                ));
                //y collisions
                let distance = details[1];
                if distance > 0.0 {
                    data_vector.push((entity_id2, 0.0, player2_component.collision_reaction_y));

                    player1_component.y_move = 0.0;
                    if player1_component.jump {
                        player2_component.points += 5;
                    } else if player2_component.jump {
                        player1_component.points += 5;
                    } else {
                        player2_component.points += 5;
                    }

                    //ToDo
                    // data_vector.push((
                    //     entity_id1,
                    //     0.0,
                    //     player1_component.collision_reaction_y * -1.0,
                    // ));
                } else {
                    commands.spawn((
                        AudioPlayer::new(audio_assets.damage.clone()),
                        PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            volume: bevy::audio::Volume::Linear(settings.hit_volume),
                            ..PlaybackSettings::ONCE
                        },
                        SoundType(AudioSettingType::Damage),
                    ));
                    data_vector.push((entity_id1, 0.0, player1_component.collision_reaction_y));

                    player2_component.y_move = 0.0;

                    if player2_component.jump {
                        player1_component.points += 5;
                    } else if player1_component.jump {
                        player2_component.points += 5;
                    } else {
                        player1_component.points += 5;
                    }

                    //ToDo
                    // data_vector.push((
                    //     entity_id2,
                    //     0.0,
                    //     player2_component.collision_reaction_y  /2.0 * -1.0,
                    // ));
                }
            }
        }
    }
    for (entity, x, y) in data_vector.iter_mut() {
        if let Ok((_entity_pom, mut transform, _collider, _player_comp)) = query.get_mut(*entity) {
            // Zmień pozycję gracza
            transform.translation.x += *x;
            transform.translation.y += *y;
        }
    }
}

pub fn player_with_coin_collision_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player, &Sprite)>,
    coin_query: Query<(Entity, &Transform, &Sprite, &Coin)>,
    audio_assets: Res<GameAudio>,
    settings: Res<AudioSettings>
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
                player.points += 25; // Dodawanie punktów
                commands.entity(coin_entity).despawn(); // usuwanie monety
            } 
        }
    }
}
