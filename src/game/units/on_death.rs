use serde::{Deserialize, Serialize};
use crate::game::units::animation::Animation;
use crate::game::units::sound::Sound;

#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct OnDeath {
    pub sound: Option<Sound>,
    pub animation: Option<Animation>,
}

impl OnDeath {
    pub fn default() -> Self {
        OnDeath {
            sound: None,
            animation: None,
        }
    }
}