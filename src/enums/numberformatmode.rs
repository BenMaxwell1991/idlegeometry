#[derive(Debug, Clone, Copy, PartialEq)]
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
