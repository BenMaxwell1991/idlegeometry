use crate::game::data::game_data::GameData;
use crate::game::objects::loot::Loot;
use crate::helper::lock_helper::acquire_lock_mut;
use std::sync::Arc;

pub fn collect_loot(loot: Vec<Loot>, game_data: Arc<GameData>) {
    let mut resources = acquire_lock_mut(&game_data.resources, "failed to acquire resources lock");

    let total_gold: f64 = loot.iter().map(|l| l.gold).sum();
    let total_exp: f64 = loot.iter().map(|l| l.exp).sum();

    if let Some(gold) = resources.get_mut("Gold") {
        *gold += total_gold;
    } else {
        resources.insert("Gold".to_string(), total_gold);
    }

    if let Some(exp) = resources.get_mut("Exp") {
        *exp += total_exp;
    } else {
        resources.insert("Exp".to_string(), total_exp);
    }
}
