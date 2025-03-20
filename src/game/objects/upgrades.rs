use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum UpgradeType {
    IncreaseDamage,
    DecreaseCooldown,
    IncreaseAOE,
    IncreaseRange,
    IncreaseSpeed,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Upgrade {
    pub upgrade_type: UpgradeType,
    pub level: u32,
}
