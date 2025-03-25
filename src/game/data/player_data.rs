use crate::game::data::resource_cost::ResourceAmount;
use crate::game::objects::upgrades::Upgrade;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub upgrades: Vec<Upgrade>,
    pub resources_persistent: ResourceAmount,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            upgrades: Vec::new(),
            resources_persistent: ResourceAmount::default(),
        }
    }
}