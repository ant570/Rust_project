use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::player::spawn::Collider;

use crate::world::platforms::PlatformMover;
use crate::world::utils::*;

const PLATFORM_SPAWN_INTERVAL: f32 = 3.0;

#[derive(Resource)]
pub struct PlatformSpawnTimer(pub Timer);

pub fn setup_platform_spawner(mut commands: Commands) {
    commands.insert_resource(PlatformSpawnTimer(Timer::from_seconds(
        PLATFORM_SPAWN_INTERVAL,
        TimerMode::Repeating,
    )));
}

pub fn platform_spawner_system(
    time: Res<Time>,
    mut timer: ResMut<PlatformSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<&Transform, With<Camera2d>>,
    images: Res<Assets<Image>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(window) = windows.single() else { return; };
    let Ok(cam_transform) = cameras.single() else { return; };

    let cam_y = cam_transform.translation.y;
    let top_screen_y = cam_y + window.height() / 2.0;

    let mut rng = rand::thread_rng();

    let left = cam_transform.translation.x - window.width() / 2.0;
    let right = cam_transform.translation.x + window.width() / 2.0;

    let x = rng.gen_range(left..right);
    let y = WORLD_HEIGHT/2.0;

    let origin = Vec3::new(x, y, 0.0);

    let texture = asset_server.load("tiles/platform.png");
    let mut platform_half_size = Vec2::new(0.0, 0.0);
    if let Some(image) = images.get(&texture){
        let platform_half_size = Vec2::new(image.width() as f32 / 2.0, image.height() as f32 / 2.0);
    }
    let decide = rng.gen_bool(0.7); // 70% szans na ruchomą platformę
    let fall_factor = 0.5;
    
    if decide {
        let amplitude = rng.gen_range(50.0..120.0); 
        let speed = rng.gen_range(1.0..4.0);        

        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_translation(origin),
            PlatformMover::horizontal(origin, amplitude, speed, fall_factor),
            Collider {
                half_size: platform_half_size,
            },
        ));
    } else {

        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_translation(origin),
            PlatformMover::falling_only(origin, fall_factor),
            Collider {
                half_size: platform_half_size,
            },
        ));
    }
}
