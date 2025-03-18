use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::collision::detect_collision::{handle_collision, rectangles_collide};
use crate::game::constants::GAME_RATE;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, GAME_IN_FOCUS, KEY_STATE, RESOURCES};
use crate::game::loops::key_state::KeyState;
use crate::game::maths::integers::int_sqrt_64;
use crate::game::maths::pos_2::{Pos2FixedPoint, INVALID_POSITION};
use crate::game::units::create_attacks::{despawn_attack, spawn_attack};
use crate::game::units::unit::{move_units_batched, remove_units};
use crate::game::units::unit_type::UnitType;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use device_query_revamped::Keycode;
use rayon::current_num_threads;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use rustc_hash::FxHashSet;
use std::cmp::max;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use crate::enums::gamestate::GameState;

pub struct GameLoop {
    pub game_data: Arc<GameData>,
    pub updated_at: Instant,
}

impl GameLoop {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self {
            game_data,
            updated_at: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let delta_time = Instant::now().duration_since(self.updated_at).as_secs_f64();
        self.updated_at = Instant::now();

        self.game_data.update_field(RESOURCES, |resources| {
            for resource in resources.iter_mut() {
                resource.update(delta_time);
            }
        });

        self.handle_input_actions();
        self.handle_animations(delta_time);
        self.handle_attacks(delta_time);
        self.handle_movement(delta_time);
    }

    fn handle_input_actions(&self) {
        let mut key_queue = self.game_data.key_queue.write().unwrap();
        let (player_id, player_position) = get_player_position(&self.game_data);

        if let Some(player_id) = player_id {
            let units = acquire_lock(&self.game_data.units, "units");

            if let Some(player) = units.get(player_id as usize).and_then(|u| u.as_ref()) {
                while let Some(key) = key_queue.pop() {
                    match key {
                        Keycode::Space => {
                            // Ensure player has an attack to use
                            if let Some(attack_name) = player.attacks.first() {
                                println!("Spawning {:?} attack at {:?}", attack_name, player_position);
                                spawn_attack(Arc::clone(&self.game_data), attack_name.clone(), player_position, Some(player_id));
                            } else {
                                println!("Player has no attacks assigned.");
                            }
                        }
                        Keycode::Escape => {
                            *self.game_data.game_state.write().unwrap() = GameState::Ready;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn handle_attack_movement(&self, delta_time: f64) {
        let player_id = self.game_data.player_id.read().unwrap();
        let unit_positions = acquire_lock(&self.game_data.unit_positions, "unit_positions");
        let attacks = acquire_lock(&self.game_data.attacks, "attacks");
        let mut attack_positions = acquire_lock_mut(&self.game_data.attack_positions, "attack_positions");

        let Some(player) = *player_id else {
            return;
        };

        attacks.iter().enumerate().for_each(|(id, attack_option)| {
            if let Some(attack) = attack_option {
                attack_positions[id].x += (attack.direction.0 * (attack.speed) as f32 * delta_time as f32) as i32;
                attack_positions[id].y += (attack.direction.1 * (attack.speed) as f32 * delta_time as f32) as i32;
            }
        });
    }

    fn handle_attack_collisions(&self) {
        let mut units_to_remove = FxHashSet::default();
        {
            let mut units = acquire_lock_mut(&self.game_data.units, "units");
            let unit_positions = acquire_lock_mut(&self.game_data.unit_positions, "unit_positions");
            let spatial_grid = acquire_lock(&self.game_data.spatial_hash_grid, "spatial_grid");
            let mut attacks = acquire_lock_mut(&self.game_data.attacks, "attacks");
            let attack_positions = acquire_lock_mut(&self.game_data.attack_positions, "attack_positions");

            for (attack_id, attack_option) in attacks.iter_mut().enumerate() {
                let Some(attack) = attack_option else { continue; };

                let attack_pos = attack_positions[attack_id];
                let nearby_unit_ids = spatial_grid.get_nearby_units(attack_pos);

                for &unit_id in &nearby_unit_ids {
                    let Some(unit) = units.get_mut(unit_id as usize).and_then(|u| u.as_mut()) else { continue; };
                    if attack.units_hit.contains(&unit_id) {
                        continue;
                    }
                    if Some(unit.id) == attack.unit_id {
                        continue;
                    }
                    let unit_pos = unit_positions[unit_id as usize];

                    if rectangles_collide(attack_pos, &attack.attack_shape, unit_pos, &unit.unit_shape) {
                        let is_dead = unit.apply_damage(attack.damage);
                        attack.units_hit.push(unit_id);
                        attack.hit_count += 1;

                        if is_dead {
                            units_to_remove.insert(unit_id);
                        }
                    }
                }
            }
        }

        let units_to_remove_vec: Vec<u32> = units_to_remove.into_iter().collect();
        if !units_to_remove_vec.is_empty() {
            remove_units(units_to_remove_vec, &self.game_data);
        }
    }

    fn handle_attacks(&self, delta_time: f64) {
        self.handle_attack_movement(delta_time);
        self.handle_attack_collisions();

        let mut expired_attacks = Vec::new();
        {
            let game_units = acquire_lock(&self.game_data.units, "game_units");
            let unit_positions = acquire_lock(&self.game_data.unit_positions, "unit_positions");
            let mut attacks = acquire_lock_mut(&self.game_data.attacks, "attacks");
            let mut attack_positions = acquire_lock_mut(&self.game_data.attack_positions, "attack_positions");

            attacks.iter_mut().enumerate().for_each(|(id, attack_option)| {
                if let Some(attack) = attack_option {
                    attack.lifetime -= delta_time as f32;

                    if let Some(unit_id) = attack.unit_id {
                        if let Some(unit) = game_units.get(unit_id as usize).and_then(|u| u.as_ref()) {
                            if unit.id as usize >= unit_positions.len() {
                                eprintln!("Error: Unit ID {} is out of bounds for unit_positions", unit.id);
                            } else {
                                // attack_positions[id] = unit_positions[unit.id as usize];
                            }
                        } else {
                            eprintln!("Warning: Attack ID {} references non-existent unit {}", id, unit_id);
                        }
                    }

                    if attack.lifetime <= 0.0 || attack.hit_count >= attack.max_targets {
                        expired_attacks.push(id as u32);
                    }
                }
            });
        }

        for attack_id in expired_attacks {
            despawn_attack(attack_id, &self.game_data);
        }
    }

    fn handle_animations(&self, delta_time: f64) {
        let mut game_units = acquire_lock_mut(&self.game_data.units, "game_units");
        game_units.par_iter_mut().for_each(|unit| {
            if let Some(unit) = unit {
                unit.animation.animation_frame =
                    (unit.animation.animation_frame + delta_time as f32 / unit.animation.animation_length.as_secs_f32()).fract();
            }
        });
        drop(game_units);

        let mut game_attacks = acquire_lock_mut(&self.game_data.attacks, "game_attacks");
        game_attacks.par_iter_mut().for_each(|attack| {
            if let Some(attack) = attack {
                attack.animation.animation_frame =
                    (attack.animation.animation_frame + delta_time as f32 / attack.animation.animation_length.as_secs_f32()).fract();
            }
        });
        drop(game_attacks);
    }

    fn handle_movement(&self, delta_time: f64) {
        let current_tab = self.game_data.get_field(CURRENT_TAB).unwrap_or(NullGameTab);
        let in_focus = self.game_data.get_field(GAME_IN_FOCUS).unwrap_or(false);
        let key_state = self.game_data.get_field(KEY_STATE).unwrap_or(Arc::new(KeyState::new()));
        let (player_id, player_position) = get_player_position(&self.game_data);

        let mut game_units = acquire_lock_mut(&self.game_data.units, "game_units");
        let unit_positions = acquire_lock(&self.game_data.unit_positions, "unit_positions");

        let game_units_len = game_units.len();
        let num_threads = current_num_threads();
        let estimated_per_thread = (game_units_len / num_threads).max(1);

        let mut unit_movements: Vec<(u32, Pos2FixedPoint, Pos2FixedPoint)> = game_units
            .par_chunks_mut(estimated_per_thread)
            .map(|chunk| {
                let mut local_buffer = Vec::with_capacity(chunk.len() * 2);
                for unit in chunk.iter_mut() {
                    if let Some(unit) = unit {
                        let movement_speed = unit.move_speed;
                        let distance: f32 = movement_speed as f32 * delta_time as f32;

                        let old_position = unit_positions[unit.id as usize];
                        let mut new_position = old_position;

                        match unit.unit_type {
                            UnitType::Player => {
                                if current_tab == GameTab::Adventure && in_focus {
                                    let dx = key_state.d.load(Ordering::Relaxed) as i32  - key_state.a.load(Ordering::Relaxed) as i32;
                                    let dy = key_state.s.load(Ordering::Relaxed) as i32 - key_state.w.load(Ordering::Relaxed) as i32;

                                    new_position.x += dx * distance as i32;
                                    new_position.y += dy * distance as i32;
                                }
                            }
                            UnitType::Enemy => {
                                let direction_vec = player_position.sub(old_position);
                                let length_squared = direction_vec.x as i64 * direction_vec.x as i64 + direction_vec.y as i64 * direction_vec.y as i64;
                                if length_squared > 0 {
                                    let abs_length = int_sqrt_64(length_squared);
                                    if abs_length > 0 {
                                        new_position.x += ((direction_vec.x as i64 * distance as i64) / abs_length) as i32;
                                        new_position.y += ((direction_vec.y as i64 * distance as i64) / abs_length) as i32;
                                    }
                                }
                            }
                        };

                        local_buffer.push((unit.id, old_position, new_position));
                    } else {
                        let invalid_position = Pos2FixedPoint::new(INVALID_POSITION, INVALID_POSITION);
                        local_buffer.push((u32::MAX, invalid_position, invalid_position));
                    }
                }

                local_buffer
            })
            .reduce(
                || Vec::with_capacity(game_units_len),
                |mut final_vec, mut thread_local_vec| {
                    final_vec.append(&mut thread_local_vec);
                    final_vec
                },
            );

        drop(game_units);
        drop(unit_positions);

        handle_collision(&mut unit_movements, &self.game_data);
        move_units_batched(&unit_movements, &self.game_data, delta_time);
    }

    pub fn start_game(mut self) {
        loop {
            let now = Instant::now();
            self.update();

            let elapsed = now.elapsed().as_millis() as u64;
            if GAME_RATE > elapsed {
                let sleep_milli = max(GAME_RATE - elapsed, 10);
                thread::sleep(Duration::from_millis(sleep_milli));
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

pub fn get_player_position(game_data: &GameData) -> (Option<u32>, Pos2FixedPoint) {
    let player_id = *game_data.player_id.read().unwrap();
    let unit_positions = game_data.unit_positions.read().unwrap();

    if let Some(id) = player_id {
        if let Some(player_pos) = unit_positions.get(id as usize) {
            return (Some(id), *player_pos);
        }
    }

    (None, Pos2FixedPoint::default())
}