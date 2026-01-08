use bevy::prelude::*;
use crate::world::utils::*;
use rand::Rng; 

#[derive(Component)]
pub struct Tile {
    pub size: Vec2,
    pub kind: TileType,
}


#[derive(PartialEq, Eq)]
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

    let tile_size = Vec2::new(TILE_SIZE, TILE_SIZE);

    let grid_width = WORLD_WIDTH;
    let grid_height = WORLD_HEIGHT;

    let start_x = -grid_width / 2.0 + TILE_SIZE / 2.0;
    let start_y = -grid_height / 2.0 + TILE_SIZE / 2.0;

    let mut rng = rand::rng();
    
    // 3 holes on the bottom
    let mut bottom_presence = vec![true; GRID_COLS as usize];
    

    for _ in 0..3 {
        let hole_width = rng.random_range(5..8); 
        for _attempt in 0..10 {
            let start = rng.random_range(5..(GRID_COLS as usize - hole_width - 5));
            let end = start + hole_width;
            
            let mut overlapping_hole = false;
            for k in start..end {
                 if !bottom_presence[k] { overlapping_hole = true; break;}
            }

            if !overlapping_hole {
                 for k in start..end { bottom_presence[k] = false; }
                 break;
            }
        }
    }

    struct Shelf {
        row: i32,
        col_start: i32,
        col_end: i32,
    }
    let mut shelves = Vec::new();
    
    let left_rows = [10, 20, 26];
    for r in left_rows {
        let length = rng.random_range(6..20);
        shelves.push(Shelf {
            row: r,
            col_start: 1,
            col_end: 1 + length,
        });
    }

    let right_rows = [15, 22];
    for r in right_rows {
        let length = rng.random_range(6..20);
        shelves.push(Shelf {
            row: r,
            col_start: GRID_COLS - 1 - length,
            col_end: GRID_COLS - 1,
        });
    }

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

            let mut spawn_type = None;

            if col == 0 || col == GRID_COLS - 1 {
                spawn_type = Some(TileType::Wall);
            } 

            else if row == 0 {
                if bottom_presence[col as usize] {
                    spawn_type = Some(TileType::Ground);
                }
            } 

            else {
                for shelf in &shelves {
                    if row == shelf.row && col >= shelf.col_start && col < shelf.col_end {
                        spawn_type = Some(TileType::Ground); 
                        break;
                    }
                }
            }

            if let Some(kind) = spawn_type {
                let texture = match kind {
                    TileType::Wall => wall_texture.clone(),
                    TileType::Ground => ground_texture.clone(),
                    TileType::Platform => panic!("Should not happen here"),
                };

                 entity.insert((
                    Sprite {
                        image: texture,
                        custom_size: Some(tile_size),
                        ..default()
                    },
                    Tile {
                        size: tile_size,
                        kind,
                    },
                ));
            }
        }
    }
}
