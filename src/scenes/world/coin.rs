use bevy::prelude::*;

use crate::scenes::world::platforms::PlatformMover;

#[derive(Component)]
pub struct AnimationConfig {
    pub frame_timer: Timer,
    pub total_frames: usize,
}
//Struktura do oznaczania monet
#[derive(Component)]
pub struct Coin;

impl AnimationConfig {
    pub fn new(frame_duration_secs: f32, total_frames: usize) -> Self {
        Self {
            frame_timer: Timer::from_seconds(frame_duration_secs, TimerMode::Repeating),
            total_frames,
        }
    }
}

pub fn spawn_coin_on_platform(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    position: Vec3,
    mover: PlatformMover,
) {
    //Wczytywanie tekstury
    let columns = 5;
    let rows = 2;
    let frame_size = UVec2::new(200, 200);

    let layout = TextureAtlasLayout::from_grid(frame_size, columns, rows, None, None);
    let texture_handle = asset_server.load("others/coin.png");
    let layout_handle = texture_atlas_layouts.add(layout);

    //Spawn monety
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(70.0, 70.0)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, position.z),
        AnimationConfig::new(0.1, 10),
        mover,
        Coin,
    ));
}

//Animacja monet
pub fn animate_coins(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in query.iter_mut() {
        config.frame_timer.tick(time.delta());

        //Zmiana klatki animacji
        if config.frame_timer.just_finished()
            && let Some(ref mut atlas) = sprite.texture_atlas
        {
            atlas.index = (atlas.index + 1) % config.total_frames;
        }
    }
}
