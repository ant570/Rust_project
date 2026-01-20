use bevy::app::{Plugin, Startup, Update};
use bevy::audio::AudioSink;

use crate::AssetServer;
use crate::AudioPlayer;
use crate::Commands;
use crate::Component;
use crate::Entity;
use crate::PlaybackSettings;
use crate::Query;
use crate::Res;
use crate::Resource;
use crate::With;
use crate::scenes::menu::settings;
use crate::scenes::menu::settings::AudioSettingType;
use crate::scenes::menu::settings::Settings;
use bevy::prelude::AudioSinkPlayback;
use bevy::prelude::DetectChanges;
use bevy::prelude::{AudioSource, Handle};

//Handle do nagrań
#[derive(Resource)]
pub struct GameAudio {
    pub coin_pickup: Handle<AudioSource>,
    pub damage: Handle<AudioSource>,
    pub fight: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,
    pub lose: Handle<AudioSource>,
    pub background: Handle<AudioSource>,
}

pub struct PlatformerGamePluginAudio;

#[derive(Component)]
pub struct SoundType(pub AudioSettingType);

impl Plugin for PlatformerGamePluginAudio {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, load_audio)
            .add_systems(Update, play_background_music)
            .add_systems(Update, sync_volume_settings)
            .add_systems(Update, cleanup_audio_flood);
    }
}

//Załadowanie plików
fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let coin_sound = asset_server.load("sounds/coin.wav");
    let damage_sound = asset_server.load("sounds/damage.wav");
    let fight_sound = asset_server.load("sounds/fight.wav");
    let jump_sound = asset_server.load("sounds/jump.wav");
    let lose_sound = asset_server.load("sounds/lose.wav");
    let background_sound = asset_server.load("music/background.mp3");
    commands.insert_resource(GameAudio {
        coin_pickup: coin_sound,
        damage: damage_sound,
        fight: fight_sound,
        jump: jump_sound,
        lose: lose_sound,
        background: background_sound,
    });
}

//Muzyka w tle
fn play_background_music(
    mut commands: Commands,
    audio_assets: Option<Res<GameAudio>>,
    existing_audio: Query<&AudioPlayer>,
    settings: Res<Settings>,
) {
    if existing_audio.is_empty()
        && let Some(assets) = audio_assets
    {
        commands.spawn((
            AudioPlayer::new(assets.background.clone()),
            PlaybackSettings {
                volume: bevy::audio::Volume::Linear(settings.music_volume),
                ..PlaybackSettings::LOOP
            },
            SoundType(AudioSettingType::Music),
        ));
    }
}

//Aktualizacja ustawień głośności.
fn sync_volume_settings(
    settings: Res<settings::Settings>,
    mut query: Query<(&SoundType, &mut PlaybackSettings, Option<&mut AudioSink>)>,
) {
    if settings.is_changed() {
        for (sound_type, mut playback, sink) in &mut query {
            let volume = match sound_type.0 {
                AudioSettingType::Music => settings.music_volume,
                AudioSettingType::Jump => settings.jump_volume,
                AudioSettingType::Coin => settings.coin_volume,
                AudioSettingType::Damage => settings.damage_volume,
                AudioSettingType::Fail => settings.fail_volume,
                AudioSettingType::Hit => settings.hit_volume,
            };
            let new_volume = bevy::audio::Volume::Linear(volume);

            if playback.volume != new_volume {
                playback.volume = new_volume;
            }
            if let Some(mut sink2) = sink
                && !sink2.empty()
            {
                sink2.set_volume(new_volume);
            }
        }
    }
}

//Usuwanie pustych, zakończonych dźwięków
pub fn cleanup_audio_flood(
    mut commands: Commands,
    query: Query<(Entity, &AudioSink, &SoundType), With<SoundType>>,
) {
    for (entity, sink, sound_type) in &query {
        if sound_type.0 != AudioSettingType::Music && sink.empty() {
            commands.entity(entity).try_despawn();
        }
    }
}
