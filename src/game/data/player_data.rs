use crate::game::objects::upgrades::Upgrade;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub upgrades: Vec<Upgrade>,
    pub resources_persistent: FxHashMap<String, f64>,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            upgrades: Vec::new(),
            resources_persistent: FxHashMap::default(),
        }
    }
}