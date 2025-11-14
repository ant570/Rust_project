use bevy::prelude::*; 

#[derive(Component)]
pub struct Ground {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct Platform {
    pub width: f32,
    pub height: f32,
    pub moving: bool,
}

#[derive(Component)]
pub struct Wall {
    pub width: f32,
    pub height: f32,
}

impl Ground {
    pub fn new(width: f32, height: f32) -> Self {
        Ground { width, height }
    }
}

impl Platform {
    pub fn new(width: f32, height: f32, moving: bool) -> Self {
        Platform {
            width,
            height,
            moving,
        }
    }
}

impl Wall {
    pub fn new(width: f32, height: f32) -> Self {
        Wall { width, height }
    }
}

pub fn spawn_map(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.3, 0.5, 0.3),
            Vec2::new(800.0, 50.0),
        ),
        Transform::from_xyz(0.0, -200.0, 0.0),
        Ground::new(800.0, 50.0),
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.5, 0.3, 0.3),
            Vec2::new(200.0, 20.0),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Platform::new(200.0, 20.0, false),
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.3, 0.3, 0.5),
            Vec2::new(50.0, 300.0),
        ),
        Transform::from_xyz(-400.0, 0.0, 0.0),
        Wall::new(50.0, 300.0),
    ));
}
