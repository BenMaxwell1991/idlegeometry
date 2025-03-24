use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::{Pos2FixedPoint, INVALID_POSITION};
use crate::game::objects::attacks::attack_defaults::{get_basic_attack, get_modified_attack};
use crate::game::objects::attacks::attack_stats::AttackName;
use crate::helper::lock_helper::acquire_lock_mut;
use rand::prelude::IndexedRandom;
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn rotate_vector(x: f32, y: f32, angle_radians: f32) -> (f32, f32) {
    let cos_theta = angle_radians.cos();
    let sin_theta = angle_radians.sin();
    (
        x * cos_theta - y * sin_theta,
        x * sin_theta + y * cos_theta,
    )
}

pub fn spawn_attack(
    game_data: Arc<GameData>,
    attack_name: AttackName,
    mut attack_origin: Pos2FixedPoint,
    unit_id: Option<u32>,
    initial_burst: bool,
) {
    let mut game_units = acquire_lock_mut(&game_data.units, "game_units");
    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "unit_positions");
    let mut empty_indexes = acquire_lock_mut(&game_data.empty_unit_indexes, "empty_unit_indexes");
    let mut attack_pools = acquire_lock_mut(&game_data.attack_pools, "attack_pools");

    let parent_unit = unit_id.and_then(|id| game_units.get(id as usize).and_then(|u| u.as_ref()));

    if !initial_burst {
        if let Some(parent_position) = unit_id.and_then(|id| unit_positions.get(id as usize)) {
            attack_origin = *parent_position;
        }
    }

    let mut base_attack = get_basic_attack(attack_name.clone());

    if let Some(parent) = parent_unit {
        base_attack = get_modified_attack(&parent.upgrades, attack_name.clone());
    }

    if let Some(attack_stats) = base_attack.attack_stats.as_ref() {
        let projectile_count = attack_stats.projectile_count;
        let spread_angle = attack_stats.spread_angle;
        let burst_count = attack_stats.burst_count;
        let burst_delay = attack_stats.burst_delay;

        // Calculate projectile directions
        let angle_step = if projectile_count > 1 {
            spread_angle / (projectile_count - 1) as f32
        } else {
            0.0
        };
        let mut directions = Vec::new();

        for i in 0..projectile_count {
            let angle = attack_stats.starting_angle + i as f32 * angle_step - (spread_angle / 2.0);
            let rotated_direction = rotate_vector(attack_stats.direction.0, attack_stats.direction.1, angle.to_radians());
            directions.push(rotated_direction);
        }

        let mut spawned_projectiles = Vec::new();

        for &direction in &directions {
            if let Some(pool) = attack_pools.get_mut(&attack_name) {
                if let Some(mut attack_unit) = pool.pop() {
                    attack_unit.parent_unit_id = unit_id;

                    if let Some(mut animation) = attack_unit.animation.as_mut() {
                        animation.animation_frame = 0.0;
                    }

                    if let Some(attack_stats) = &mut attack_unit.attack_stats {
                        attack_stats.enabled = true;
                        attack_stats.elapsed = 0.0;
                        attack_stats.units_hit.clear();
                        attack_stats.direction = direction;
                        attack_stats.hit_count = 0;
                    }

                    let attack_id = if let Some(reuse_index) = empty_indexes.pop() {
                        attack_unit.id = reuse_index;
                        game_units[reuse_index as usize] = Some(attack_unit);
                        unit_positions[reuse_index as usize] = attack_origin;
                        reuse_index
                    } else {
                        let new_index = game_units.len() as u32;
                        attack_unit.id = new_index;
                        game_units.push(Some(attack_unit));
                        unit_positions.push(attack_origin);
                        new_index
                    };

                    spawned_projectiles.push(attack_id);
                }
            }
        }

        if initial_burst && burst_count > 1 && burst_delay > 0.0 {
            for i in 0..(burst_count - 1) {
                let game_data_clone = Arc::clone(&game_data);
                spawn(move || {
                    sleep(Duration::from_secs_f32(burst_delay * (i + 1) as f32));
                    spawn_attack(game_data_clone, attack_name, attack_origin, unit_id, false);
                });
            }
        }
    }
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
