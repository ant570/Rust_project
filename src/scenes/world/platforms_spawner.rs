use crate::scenes::world::platforms::PlatformMover;
use crate::scenes::world::spawn::Tile;
use crate::scenes::world::spawn::TileType;
use crate::scenes::world::utils::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

const PLATFORM_SPAWN_INTERVAL: f32 = 3.0;

#[derive(Resource, Default)]
pub struct PlatformSpawnTimer(pub Timer);

pub fn setup_platform_spawner(mut commands: Commands) {
    commands.insert_resource(PlatformSpawnTimer(Timer::from_seconds(
        PLATFORM_SPAWN_INTERVAL,
        TimerMode::Repeating,
    )));
}

#[allow(clippy::too_many_arguments)]
pub fn platform_spawner_system(
    time: Res<Time>,
    mut timer: ResMut<PlatformSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<&Transform, With<Camera2d>>,
    images: Res<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Ok(cam_transform) = cameras.single() else {
        return;
    };

    //let cam_y = cam_transform.translation.y;
    //let top_screen_y = cam_y + window.height() / 2.0;

    let mut rng = rand::rng();

    let left = cam_transform.translation.x - window.width() / 2.0;
    let right = cam_transform.translation.x + window.width() / 2.0;

    let x = rng.random_range(left..right);
    let y = WORLD_HEIGHT / 2.0;

    let origin = Vec3::new(x, y, 0.0);

    let texture = asset_server.load("tiles/platform.png");
    let mut platform_size = Vec2::new(0.0, 0.0);
    if let Some(image) = images.get(&texture) {
        platform_size = Vec2::new(image.width() as f32, image.height() as f32);
    }

    let decide = rng.random_bool(0.7); // 70% szans na ruchomą platformę
    let fall_factor = 0.5;

    let scale_x = rng.random_range(1.0..2.5);

    if platform_size == Vec2::ZERO {
        platform_size = Vec2::new(
            crate::scenes::world::utils::TILE_SIZE,
            crate::scenes::world::utils::TILE_SIZE * 0.5,
        );
    }
    platform_size.x *= scale_x;

    let mut sprite = Sprite::from_image(texture.clone());
    sprite.custom_size = Some(platform_size);

    let coin = rng.random_bool(0.3); // 30% szans na monetę na platformie

    if decide {
        let amplitude = rng.random_range(50.0..120.0);
        let speed = rng.random_range(1.0..4.0);

        let mover = PlatformMover::horizontal(origin, amplitude, speed, fall_factor);
        commands.spawn((
            sprite,
            Transform::from_translation(origin),
            mover,
            Tile {
                size: platform_size,
                kind: TileType::Platform,
            },
        ));
        if coin {
            crate::scenes::world::coin::spawn_coin_on_platform(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                Vec3::new(x, y + 60.0, 1.0),
                crate::scenes::world::platforms::PlatformMover::horizontal(
                    origin,
                    amplitude,
                    speed,
                    fall_factor,
                ),
            );
        }
    } else {
        let mover = PlatformMover::falling_only(origin, fall_factor);
        commands.spawn((
            sprite,
            Transform::from_translation(origin),
            mover,
            Tile {
                size: platform_size,
                kind: TileType::Platform,
            },
        ));
        if coin {
            crate::scenes::world::coin::spawn_coin_on_platform(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                Vec3::new(x, y + 60.0, 1.0),
                crate::scenes::world::platforms::PlatformMover::falling_only(origin, fall_factor),
            );
        }
    }
}
