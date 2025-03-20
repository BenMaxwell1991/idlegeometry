use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::{Pos2FixedPoint, INVALID_POSITION};
use crate::game::objects::attacks::attack_defaults::{get_basic_attack, get_modified_attack};
use crate::game::objects::attacks::attack_stats::AttackName;
use crate::helper::lock_helper::acquire_lock_mut;
use rand::prelude::IndexedRandom;
use std::sync::Arc;

pub fn spawn_attack(
    game_data: Arc<GameData>,
    attack_name: AttackName,
    attack_origin: Pos2FixedPoint,
    unit_id: Option<u32>,
) -> Option<u32> {
    let mut game_units = game_data.units.write().unwrap();
    let mut unit_positions = game_data.unit_positions.write().unwrap();
    let mut empty_indexes = game_data.empty_unit_indexes.write().unwrap();
    let mut attack_pools = game_data.attack_pools.write().unwrap();

    let parent_unit = unit_id.and_then(|id| game_units.get(id as usize).and_then(|u| u.as_ref()));

    if let Some(pool) = attack_pools.get_mut(&attack_name) {
        if let Some(mut attack_unit) = pool.pop() {

            if let Some(parent) = parent_unit {
                attack_unit = get_modified_attack(&parent.upgrades, attack_name.clone());
            }

            attack_unit.parent_unit_id = unit_id;
            attack_unit.animation.animation_frame = 0.0;

            if let Some(attack_stats) = &mut attack_unit.attack_stats {
                attack_stats.enabled = true;
                attack_stats.elapsed = 0.0;
                attack_stats.units_hit.clear();
                attack_stats.direction = (0.0, 1.0);
                attack_stats.hit_count = 0;
            } else {
                attack_unit.attack_stats = get_basic_attack(attack_name.clone()).attack_stats;
            }

            let attack_id = if let Some(reuse_index) = empty_indexes.pop() {
                attack_unit.id = reuse_index;
                game_units[reuse_index as usize] = Some(attack_unit);
                unit_positions[reuse_index as usize] = attack_origin;
                Some(reuse_index)
            } else {
                let new_index = game_units.len() as u32;
                attack_unit.id = new_index;
                game_units.push(Some(attack_unit));
                unit_positions.push(attack_origin);
                Some(new_index)
            };
            return attack_id;
        }
    }

    None
}

pub fn despawn_attack(attack_id: u32, game_data: &GameData) {
    let mut game_units = acquire_lock_mut(&game_data.units, "game_units");
    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "unit_positions");
    let mut empty_indexes = acquire_lock_mut(&game_data.empty_unit_indexes, "empty_unit_indexes");
    let mut attack_pools = acquire_lock_mut(&game_data.attack_pools, "attack_pools");

    if let Some(unit) = game_units.get_mut(attack_id as usize) {
        if let Some(mut attack_unit) = unit.take() {
            unit_positions[attack_id as usize] = Pos2FixedPoint::new(INVALID_POSITION, INVALID_POSITION);
            empty_indexes.push(attack_id);

            if let Some(attack_stats) = &mut attack_unit.attack_stats {
                attack_stats.enabled = false;
                attack_stats.elapsed = 0.0;
                attack_stats.units_hit.clear();
            }

            if let Some(attack_stats) = &attack_unit.attack_stats {
                attack_pools.get_mut(&attack_stats.name).unwrap().push(attack_unit);
            }
        }
    }
}
