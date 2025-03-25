use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct ExperienceData {
    pub level: u64,
    pub experience: f64
}

impl Default for ExperienceData {
    fn default() -> Self {
        Self {
            level: 0,
            experience: 0.0,
        }
    }
}