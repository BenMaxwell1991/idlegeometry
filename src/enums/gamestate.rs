use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Ready,
    Playing,
    Paused,
    Dead,
    Quitting,
}