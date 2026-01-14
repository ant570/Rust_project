use bevy::app::{Plugin, Startup, Update};

use crate::AssetServer;
use crate::AudioPlayer;
use crate::Commands;
use crate::PlaybackSettings;
use crate::Query;
use crate::Res;
use crate::Resource;
use bevy::prelude::{AudioSource, Handle};

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

impl Plugin for PlatformerGamePluginAudio {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, load_audio)
            .add_systems(Update, play_background_music);
    }
}
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

fn play_background_music(
    mut commands: Commands,
    audio_assets: Option<Res<GameAudio>>,
    existing_audio: Query<&AudioPlayer>,
) {
    if existing_audio.is_empty()
        && let Some(assets) = audio_assets
    {
        commands.spawn((
            AudioPlayer::new(assets.background.clone()),
            PlaybackSettings::LOOP,
        ));
    }
}
