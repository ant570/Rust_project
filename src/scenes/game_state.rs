use crate::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing,
    HowToPlay1,
    HowToPlay2,
    Paused,
}