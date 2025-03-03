use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NumberFormatMode {
    Standard,
    Engineering,
    Exponential,
}

impl NumberFormatMode {
    pub fn default() -> Self {
        NumberFormatMode::Engineering
    }
}
