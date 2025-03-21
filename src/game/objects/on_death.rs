use crate::game::objects::animation::Animation;
use crate::game::objects::sound::Sound;

#[derive(Clone, Debug)]
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