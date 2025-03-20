use crate::ui::sound::music_player::MONSTER_DEATH_01;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sound {
    pub name: String,
    pub volume: f32,
}

impl Sound {
    pub fn death_01_default() -> Self {
        Self {
            name: MONSTER_DEATH_01.to_string(),
            volume: 0.1,
        }
    }
}

impl PartialEq<> for Sound {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Sound {}

impl Hash for Sound {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}