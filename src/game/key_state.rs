use std::sync::atomic::AtomicBool;

#[derive(Debug)]
pub struct KeyState {
    pub w: AtomicBool,
    pub a: AtomicBool,
    pub s: AtomicBool,
    pub d: AtomicBool,
}

impl KeyState {
    pub fn new() -> Self {
        Self {
            w: AtomicBool::new(false),
            a: AtomicBool::new(false),
            s: AtomicBool::new(false),
            d: AtomicBool::new(false),
        }
    }
}
