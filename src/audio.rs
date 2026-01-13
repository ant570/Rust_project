use bevy::app::{Plugin, Startup};

use crate::AssetServer;
use crate::Res;
use crate::Commands;
use bevy::prelude::{Handle, AudioSource};
use crate::Resource;

#[derive(Resource)]

pub struct GameAudio {
    pub coin_pickup: Handle<AudioSource>,
    pub damage: Handle<AudioSource>,
    pub fight: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,
    pub lose: Handle<AudioSource>,
}

pub struct PlatformerGamePluginAudio;

impl Plugin for PlatformerGamePluginAudio {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, load_audio);
    }
}
fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let coin_sound = asset_server.load("sounds/coin.wav");
    let damage_sound = asset_server.load("sounds/damage.wav");
    let fight_sound = asset_server.load("sounds/fight.wav");
    let jump_sound = asset_server.load("sounds/jump.wav");
    let lose_sound = asset_server.load("sounds/lose.wav");
    commands.insert_resource(GameAudio {
        coin_pickup: coin_sound,
        damage: damage_sound,
        fight: fight_sound,
        jump: jump_sound,
        lose: lose_sound,
    });
}