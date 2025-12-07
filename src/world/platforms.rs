use bevy::prelude::*;

#[derive(Component)]
pub struct PlatformMover {
    pub amplitude: f32,
    pub speed: f32,
    pub origin: Vec3,
    pub horizontal: bool,
}

impl PlatformMover {
    pub fn horizontal(origin: Vec3, amplitude: f32, speed: f32) -> Self {
        Self {
            amplitude,
            speed,
            origin,
            horizontal: true,
        }
    }

    pub fn vertical(origin: Vec3, amplitude: f32, speed: f32) -> Self {
        Self {
            amplitude,
            speed,
            origin,
            horizontal: false,
        }
    }
}


pub fn move_platforms_system(
    time: Res<Time>,
    mut query: Query<(&PlatformMover, &mut Transform)>,
) {
    let t = time.elapsed_secs();

    for (mover, mut transform) in &mut query {
        let offset = (t * mover.speed).sin() * mover.amplitude;

        if mover.horizontal {
            transform.translation.x = mover.origin.x + offset;
            transform.translation.y = mover.origin.y;
        } else {
            transform.translation.y = mover.origin.y + offset;
            transform.translation.x = mover.origin.x;
        }
    }
}
