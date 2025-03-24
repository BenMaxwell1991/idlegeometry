use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAmount {
    pub gold: f64,
    pub ruby: f64,
    pub gemstone: f64,
    pub experience: f64,
    pub fire: f64,
}

impl Default for ResourceAmount {
    fn default() -> Self {
        Self {
            gold: 0.0,
            ruby: 0.0,
            gemstone: 0.0,
            experience: 0.0,
            fire: 0.0,
        }
    }
}