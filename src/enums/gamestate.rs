use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Ready,
    Playing,
    Paused,
    Dead,
    Quitting,
}

impl GameState {
    pub fn is_game_active(&self) -> bool {
        match self {
            GameState::Ready => { false }
            GameState::Playing => {true }
            GameState::Paused => { false }
            GameState::Dead => { false }
            GameState::Quitting => { false }
        }
    }
}