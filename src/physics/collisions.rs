use bevy::prelude::*;
use crate::player::spawn::Player;
use crate::world::spawn::{Tile, TileType};

pub fn aabb_collision(
    pos_a: Vec2,
    size_a: Vec2,
    pos_b: Vec2,
    size_b: Vec2,
) -> bool {
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
    mut player_query: Query<(&mut Transform, &Sprite, &Player)>,
    tile_query: Query<(&Transform, &Sprite, &Tile), Without<Player>>,
) {
    for (mut player_transform, player_sprite, _player) in &mut player_query {
        let mut player_pos = player_transform.translation.truncate();   // bez pozycji z osi z
        let player_size = player_sprite.custom_size.unwrap_or(Vec2::ZERO);

        for (tile_transform, tile_sprite, tile) in &tile_query {
            let tile_size = tile_sprite.custom_size.unwrap_or(Vec2::ZERO);
            let tile_pos = tile_transform.translation.truncate();

            if !aabb_collision(player_pos, player_size, tile_pos, tile_size) {
                continue;
            }

            match tile.kind {
                TileType::Ground | TileType::Wall => {
                    let delta = player_pos - tile_pos;

                    let combined_half = (player_size + tile_size) / 2.0;
                    let overlap_x = combined_half.x - delta.x.abs();
                    let overlap_y = combined_half.y - delta.y.abs();  // sprawdzenie nakladania sie

                    if overlap_x < overlap_y {
                        if delta.x > 0.0 {
                            player_pos.x += overlap_x;
                        } else {
                            player_pos.x -= overlap_x;
                        }
                    } else {
                        if delta.y > 0.0 {
                            player_pos.y += overlap_y;
                        } else {
                            player_pos.y -= overlap_y;
                        }
                    }
                    player_transform.translation.x = player_pos.x;
                    player_transform.translation.y = player_pos.y;

                }
                TileType::Platform => {
                    // TODO
                }
            }
        }
    }
}
