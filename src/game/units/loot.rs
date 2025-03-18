use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Loot {
    pub gold: f64,
    pub exp: f64,
}

impl Loot {
    pub fn default() -> Self {
        Self {
            gold: 0.0,
            exp: 0.0,
        }
    }
}