use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::world::platforms::PlatformMover;

const TILE_SIZE: f32 = 50.0;

const GRID_COLS: i32 = 50;
const GRID_ROWS: i32 = 40;

const WORLD_WIDTH: f32 = GRID_COLS as f32 * TILE_SIZE;
const WORLD_HEIGHT: f32 = GRID_ROWS as f32 * TILE_SIZE;

#[derive(Component)]
pub struct Tile {
    pub size: Vec2,
    pub kind: TileType,
}


pub enum TileType {
    Ground,
    Platform,
    Wall,
}

pub fn spawn_camera(mut commands: Commands) {
    let projection = Projection::from(OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::FixedVertical {
            viewport_height: WORLD_HEIGHT,
        },
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((
        Camera2d,
        projection,
    ));
}

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ground_texture: Handle<Image> = asset_server.load("tiles/stone.png");
    let wall_texture: Handle<Image> = asset_server.load("tiles/wall.png");
    let platform_texture: Handle<Image> = asset_server.load("tiles/platform.png");

    let tile_size = Vec2::new(TILE_SIZE, TILE_SIZE);

    let grid_width = WORLD_WIDTH;
    let grid_height = WORLD_HEIGHT;

    let start_x = -grid_width / 2.0 + TILE_SIZE / 2.0;
    let start_y = -grid_height / 2.0 + TILE_SIZE / 2.0;

    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let x = start_x + col as f32 * TILE_SIZE;
            let y = start_y + row as f32 * TILE_SIZE;

            let mut entity = commands.spawn((
                Sprite {
                    custom_size: Some(tile_size),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));

            if row == 0 {
                entity.insert((
                    Sprite {
                        image: ground_texture.clone(),
                        custom_size: Some(tile_size),
                        ..default()
                    },
                    Tile {
                        size: tile_size,
                        kind: TileType::Ground,
                    },
                ));
            }

            else if col == 0 {
                entity.insert((
                    Sprite {
                        image: wall_texture.clone(),
                        custom_size: Some(tile_size),
                        ..default()
                    },
                    Tile {
                        size: tile_size,
                        kind: TileType::Wall,
                    },
                ));
            }

            else if col == GRID_COLS - 1 {
                entity.insert((
                    Sprite {
                        image: wall_texture.clone(),
                        custom_size: Some(tile_size),
                        ..default()
                    },
                    Tile {
                        size: tile_size,
                        kind: TileType::Wall,
                    },
                ));
            }
        }
    }


    // platforms 
    let platform_positions_static = vec![
        (5, 5),
        (15, 15),
        (25, 25),
    ];

    for (col, row) in platform_positions_static {
        let x = start_x + col as f32 * TILE_SIZE;
        let y = start_y + row as f32 * TILE_SIZE;

        let platform_size = Vec2::new(3.0 * TILE_SIZE, TILE_SIZE);

        commands.spawn((
            Sprite {
                image: platform_texture.clone(),
                custom_size: Some(platform_size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Tile {
                size: platform_size,
                kind: TileType::Platform,
            },
        ));
    }

    let platform_positions_moving = vec![
        (10, 10),
        (20, 20),
        (30, 30),
    ];

    for (col, row) in platform_positions_moving {
        let x = start_x + col as f32 * TILE_SIZE;
        let y = start_y + row as f32 * TILE_SIZE;

        let platform_size = Vec2::new(3.0 * TILE_SIZE, TILE_SIZE);

        commands.spawn((
            Sprite {
                image: platform_texture.clone(),
                custom_size: Some(platform_size),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Tile {
                size: platform_size,
                kind: TileType::Platform,
            },
            PlatformMover::horizontal(Vec3::new(x, y, 0.0), 50.0, 2.0)
        ));
    }
}
