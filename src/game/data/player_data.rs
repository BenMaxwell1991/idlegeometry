use crate::game::data::resource_cost::ResourceAmount;
use crate::game::objects::upgrades::Upgrade;
use serde::{Deserialize, Serialize};
use crate::game::data::experience_data::ExperienceData;
use crate::helper::lock_helper::acquire_lock_mut;
use crate::ui::component::widget::lair_object::{get_lair_object, LairObject};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub upgrades: Vec<Upgrade>,
    pub resources_persistent: ResourceAmount,
    pub adventure_provisioning: ResourceAmount,
    pub lair_objects: Vec<LairObject>,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            upgrades: Vec::new(),
            resources_persistent: ResourceAmount::default(),
            adventure_provisioning: ResourceAmount::default_provisions(),
            lair_objects: Vec::new(),
        }
    }
}

impl PlayerData {
    pub fn initialize_lair_objects() -> Vec<LairObject> {
        let mut objects = Vec::new();

        for i in 0..2 {
            let mut object = get_lair_object(i, ExperienceData::default());
            objects.push(object);
        }

        objects
    }
}