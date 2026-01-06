use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationConfig {
    pub frame_timer: Timer,
    pub total_frames: usize,
}

impl AnimationConfig {
    pub fn new(frame_duration_secs: f32, total_frames: usize) -> Self {
        Self {
            frame_timer: Timer::from_seconds(frame_duration_secs, TimerMode::Repeating),
            total_frames,
        }
    }
}
pub fn spawn_initial_coins(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let columns= 5;
    let rows = 2;
    let frame_size = Vec2::new(200.0, 200.0);

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(200, 200),
        columns, 
        rows,
        None,
        None,
    );

    let texture_handle = asset_server.load("others/coin.png");
    let layout_handle = texture_atlas_layouts.add(layout);
    println!("Spawning initial coins");
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 1.0),
        AnimationConfig::new(0.1, 10),
    ));
}

pub fn animate_coin(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in query.iter_mut() {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % config.total_frames;
            }
        }
    }
}