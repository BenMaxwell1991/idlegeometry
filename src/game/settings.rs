use crate::enums::numberformatmode::NumberFormatMode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub number_format_mode: NumberFormatMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            number_format_mode: NumberFormatMode::default(),
        }
    }
}