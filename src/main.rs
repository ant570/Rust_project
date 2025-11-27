use bevy::prelude::*;
mod game;
use crate::game::position::Position2;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::game::PlatformerGamePlugin)
        .run();

    let pos = Position2::new(10.0, 20.0);
    pos.to_str();
}