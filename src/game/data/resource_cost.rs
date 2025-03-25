use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAmount {
    pub food: Option<f64>,
    pub gold: Option<f64>,
    pub exp: Option<f64>,
    pub ruby: Option<f64>,
    pub gemstone: Option<f64>,
    pub experience: Option<f64>,
    pub fire: Option<f64>,
}

impl Default for ResourceAmount {
    fn default() -> Self {
        Self {
            food: None,
            gold: None,
            exp: None,
            ruby: None,
            gemstone: None,
            experience: None,
            fire: None,
        }
    }
}