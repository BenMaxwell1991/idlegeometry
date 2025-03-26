use std::sync::Arc;
use crate::game::data::experience_data::ExperienceData;
use crate::game::data::resource_cost::ResourceAmount;
use crate::game::objects::upgrades::Upgrade;
use crate::ui::component::widget::lair_object::{get_lair_object, LairObject};
use serde::{Deserialize, Serialize};
use crate::game::data::game_data::GameData;

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
    pub fn initialize_lair_objects(game_data: Arc<GameData>) -> Vec<LairObject> {
        let mut objects = Vec::new();

        for i in 0..15 {
            let mut object = get_lair_object(Arc::clone(&game_data), i, ExperienceData::default());
            objects.push(object);
        }

        objects
    }

    pub fn try_purchase_lair_object(&mut self, name: &str) -> bool {
        if let Some(obj) = self.lair_objects.iter_mut().find(|o| o.name == name) {
            if ResourceAmount::can_afford(&self.resources_persistent, &obj.production_cost) {
                ResourceAmount::pay_cost(&mut self.resources_persistent, &obj.production_cost);
                obj.quantity += 1;

                // Unlock the next lair object if available
                if let Some(next_obj) = self.lair_objects.iter_mut().find(|o| !o.unlocked) {
                    next_obj.unlocked = true;
                }

                return true;
            }
        }
        false
    }
}