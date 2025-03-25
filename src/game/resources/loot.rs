use crate::game::data::game_data::GameData;
use crate::game::objects::loot::Loot;
use crate::helper::lock_helper::acquire_lock_mut;
use std::sync::Arc;

pub fn collect_loot(loot: Vec<Loot>, game_data: Arc<GameData>) {
    let mut resource_amounts = acquire_lock_mut(&game_data.resource_amounts, "resource_amounts");

    let total_gold: f64 = loot.iter().map(|l| l.gold).sum();
    let total_exp: f64 = loot.iter().map(|l| l.exp).sum();

    *resource_amounts.gold.get_or_insert(total_gold) += total_gold;
    *resource_amounts.exp.get_or_insert(total_exp) += total_exp;
}
