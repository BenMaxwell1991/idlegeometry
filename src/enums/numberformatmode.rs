#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberFormatMode {
    Standard,    // 123,456,789
    Scientific,  // 1.23e6
    Exponential, // 1.23456e+6
}

impl NumberFormatMode {
    pub fn default() -> Self {
        NumberFormatMode::Scientific
    }
}
