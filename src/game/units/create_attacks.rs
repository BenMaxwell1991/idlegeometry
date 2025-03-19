use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::Pos2FixedPoint;
use crate::game::units::attack::{Attack, AttackName};
use crate::helper::lock_helper::acquire_lock_mut;
use rand::prelude::IndexedRandom;
use std::sync::Arc;

pub fn spawn_attack(
    game_data: Arc<GameData>,
    attack_name: AttackName,
    attack_origin: Pos2FixedPoint,
    unit_id: Option<u32>,
) -> Option<u32> {
    let mut attack_pools = game_data.attack_pools.write().unwrap();
    let mut attacks = game_data.attacks.write().unwrap();
    let mut attack_positions = game_data.attack_positions.write().unwrap();
    let mut empty_indexes = game_data.empty_attack_indexes.write().unwrap();
    let units = game_data.units.read().unwrap();

    let unit = unit_id.and_then(|id| units.get(id as usize).and_then(|u| u.as_ref()));

    if let Some(pool) = attack_pools.get_mut(&attack_name) {
        if let Some(mut attack) = pool.pop() {
            attack.enabled = true;
            attack.attack_origin = attack_origin;
            attack.animation.animation_frame = 0.0;
            attack.elapsed = 0.0;
            attack.units_hit = Vec::new();
            attack.origin_unit_id = unit_id;

            if let Some(unit) = unit {
                let modified_attack = Attack::get_modified_attack(&unit.upgrades, attack_name);
                attack.damage = modified_attack.damage;
                attack.range = modified_attack.range;
                attack.cooldown = modified_attack.cooldown;
                attack.speed = modified_attack.speed;
                attack.area = modified_attack.area;
                attack.lifetime = modified_attack.lifetime;
                attack.attack_shape = modified_attack.attack_shape;
            }

            let attack_id = if let Some(reuse_index) = empty_indexes.pop() {
                attack.id = reuse_index;
                attacks[reuse_index as usize] = Some(attack);
                attack_positions[reuse_index as usize] = attack_origin;
                Some(reuse_index)
            } else {
                let new_index = attacks.len() as u32;
                attack.id = new_index;
                attacks.push(Some(attack));
                attack_positions.push(attack_origin);
                Some(new_index)
            };

            return attack_id;
        }
    }

    None
}


pub fn despawn_attack(attack_id: u32, game_data: &GameData) {
    let mut attacks = acquire_lock_mut(&game_data.attacks, "attacks");
    let mut attack_positions = acquire_lock_mut(&game_data.attack_positions, "attack_positions");
    let mut empty_indexes = acquire_lock_mut(&game_data.empty_attack_indexes, "empty_attack_indexes");
    let mut attack_pools = acquire_lock_mut(&game_data.attack_pools, "attack_pools");

    if let Some(attack) = attacks.get_mut(attack_id as usize) {
        if let Some(attack) = attack.take() {
            attack_positions[attack_id as usize] = Pos2FixedPoint::new(0, 0);
            empty_indexes.push(attack_id);

            let mut reusable_attack = attack.clone();
            reusable_attack.enabled = false;
            reusable_attack.lifetime = 0.0;
            attack_pools.get_mut(&reusable_attack.name).unwrap().push(reusable_attack);
        }
    }
}
