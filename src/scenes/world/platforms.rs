use crate::scenes::world::utils::TILE_SIZE;
use crate::scenes::world::utils::WORLD_HEIGHT;
use bevy::prelude::*;

const PLATFORM_BASE_FALL_SPEED: f32 = 150.0;
const DESPAWN_Y: f32 = -WORLD_HEIGHT / 2.0 + TILE_SIZE * 1.5;

#[derive(Component, Clone, Copy)]
pub struct PlatformMover {
    pub origin: Vec3,
    pub horizontal: bool,
    pub amplitude: f32,   // jak daleko ma iść na boki
    pub speed: f32,       // jak szybko ma się ruszać na boki
    pub fall_factor: f32, // jak szybko ma spadać
}

impl PlatformMover {
    pub fn horizontal(origin: Vec3, amplitude: f32, speed: f32, fall_factor: f32) -> Self {
        Self {
            origin,
            horizontal: true,
            amplitude,
            speed,
            fall_factor,
        }
    }

    pub fn falling_only(origin: Vec3, fall_factor: f32) -> Self {
        Self {
            origin,
            horizontal: false,
            amplitude: 0.0,
            speed: 0.0,
            fall_factor,
        }
    }
}

pub fn move_platforms_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &PlatformMover, &mut Transform)>,
) {
    let t = time.elapsed_secs(); // do sinusa
    let dt = time.delta_secs(); // do spadania

    for (entity, mover, mut transform) in &mut query {
        let horizontal_offset = if mover.horizontal {
            (t * mover.speed).sin() * mover.amplitude
        } else {
            0.0
        };

        transform.translation.x = mover.origin.x + horizontal_offset;

        transform.translation.y -= PLATFORM_BASE_FALL_SPEED * mover.fall_factor * dt;

        if transform.translation.y < DESPAWN_Y {
            commands.entity(entity).try_despawn();
        }
    }
}
