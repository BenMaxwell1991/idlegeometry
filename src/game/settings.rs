use crate::enums::numberformatmode::NumberFormatMode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub number_format_mode: NumberFormatMode,
    pub window_width: f32,
    pub window_height: f32,
    pub vsync: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            number_format_mode: NumberFormatMode::default(),
            window_width: 1280.0,  // Default window size
            window_height: 720.0,
            vsync: true, // Default VSync setting
        }
    }
}
