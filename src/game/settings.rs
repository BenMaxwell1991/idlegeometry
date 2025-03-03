use crate::enums::numberformatmode::NumberFormatMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub number_format_mode: NumberFormatMode,
    pub window_width: f32,
    pub window_height: f32,
    pub vsync: bool,
    pub autosave_interval: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            number_format_mode: NumberFormatMode::default(),
            window_width: 1280.0,
            window_height: 720.0,
            vsync: true,
            autosave_interval: 5,
        }
    }
}
