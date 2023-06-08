use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum EngineState {
    #[default]
    LoadingAssets,
    // MainMenu,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}
