use crate::enums::gamestate::GameState;
use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::collision::detect_collision::handle_collision;
use crate::game::constants::GAME_RATE;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, GAME_IN_FOCUS, KEY_STATE, RESOURCES};
use crate::game::loops::key_state::KeyState;
use crate::game::maths::integers::int_sqrt_64;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE, INVALID_POSITION};
use crate::game::objects::attacks::attack_defaults::{get_basic_attack, get_modified_attack};
use crate::game::objects::attacks::create_attacks::{despawn_attack, spawn_attack};
use crate::game::objects::game_object::move_units_batched;
use crate::game::objects::object_type::ObjectType;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use device_query_revamped::Keycode;
use rand::prelude::IndexedRandom;
use rayon::current_num_threads;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use std::cmp::max;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use smallvec::SmallVec;
use crate::game::objects::attacks::attack_stats::AttackName;

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
        let player_dead = acquire_lock(&self.game_data.player_dead, "player_dead").clone();
        let game_loop_active = self.game_data.game_loop_active.load(Ordering::Relaxed);

        let mut key_queue = self.game_data.key_queue.write().unwrap();
        let (player_id, player_position) = get_player_position(&self.game_data);

        if let Some(player_id) = player_id {
            let attack_name = Some(AttackName::LightningBolt);
            while let Some(key) = key_queue.pop() {
                match key {
                    Keycode::Space => {
                        if !player_dead && game_loop_active {
                            if let Some(attack_name) = &attack_name {
                                println!("Spawning {:?} attack at {:?}", attack_name.clone(), player_position);
                                spawn_attack(Arc::clone(&self.game_data), attack_name.clone(), player_position, Some(player_id), true);
                            } else {
                                println!("Player has no attacks assigned.");
                            }
                        }
                    }
                    Keycode::Escape => {
                        self.game_data.set_game_state(GameState::Ready);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_attacks(&self, delta_time: f64) {
        let mut expired_attacks = Vec::new();
        let mut attacks_to_spawn: Vec<(AttackName, Pos2FixedPoint, u32)> = Vec::new();

        {
            let mut game_units = acquire_lock_mut(&self.game_data.units, "game_units");
            let mut unit_positions = acquire_lock_mut(&self.game_data.unit_positions, "unit_positions");

            // **Handle attack lifetimes** → Remove expired attacks
            for unit in game_units.iter_mut().flatten() {
                if unit.object_type == ObjectType::Attack {
                    if let Some(attack_stats) = &mut unit.attack_stats {
                        attack_stats.elapsed += delta_time as f32;

                        if attack_stats.elapsed >= attack_stats.lifetime || attack_stats.hit_count >= attack_stats.max_targets {
                            expired_attacks.push(unit.id);
                        }
                    }
                }
            }

            for unit in game_units.iter_mut().flatten() {
                if unit.object_type != ObjectType::Attack {
                    let unit_position = unit_positions[unit.id as usize];
                    for (attack_name, cooldown) in unit.attack_cooldowns.iter_mut() {
                        *cooldown -= delta_time as f32;
                        if *cooldown <= 0.0 {
                            let attack = get_modified_attack(&unit.upgrades, attack_name.clone());
                            if let Some(attack_stats) = attack.attack_stats.as_ref() {
                                if !attack_stats.proximity_attack {
                                    attacks_to_spawn.push((attack_name.clone(), unit_position, unit.id));
                                    *cooldown = attack_stats.cooldown;
                                }
                            }
                        }
                    }
                }
            }
        }

        for attack_id in expired_attacks {
            despawn_attack(attack_id, &self.game_data);
        }

        for (attack_name, unit_position, unit_id) in attacks_to_spawn {
            spawn_attack(Arc::clone(&self.game_data), attack_name.clone(), unit_position, Some(unit_id), true);
        }
    }

    fn handle_animations(&self, delta_time: f64) {
        let mut game_units = acquire_lock_mut(&self.game_data.units, "game_units");
        game_units.par_iter_mut().for_each(|unit| {
            if let Some(unit) = unit {
                if let Some(mut animation) = unit.animation.as_mut() {
                    if !animation.fixed_frame_index.is_some() {
                        animation.animation_frame =
                            (animation.animation_frame + delta_time as f32 / animation.animation_length.as_secs_f32()).fract();
                    }
                }
            }
        });
        drop(game_units);
    }

    fn handle_movement(&self, delta_time: f64) {
        let (player_id, mut player_position) = get_player_position(&self.game_data);

        if let player_dead = acquire_lock(&self.game_data.player_dead, "player_dead").clone(){
            if let Some(position) = *acquire_lock(&self.game_data.player_position, "player_position") {
                player_position = position;
            }
        }

        let mut game_units = acquire_lock_mut(&self.game_data.units, "game_units");
        let unit_positions = acquire_lock(&self.game_data.unit_positions, "unit_positions");
        let current_tab = self.game_data.get_field(CURRENT_TAB).unwrap_or(NullGameTab).clone();
        let in_focus = self.game_data.get_field(GAME_IN_FOCUS).unwrap_or(false).clone();
        let key_state = self.game_data.get_field(KEY_STATE).unwrap_or(Arc::new(KeyState::new())).clone();

        let pickup_radius = player_id
            .and_then(|id| game_units.get(id as usize))
            .and_then(|unit_option| unit_option.as_ref())
            .and_then(|player| player.pickup_radius)
            .unwrap_or(0).clone();

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

                        match unit.object_type {
                            ObjectType::Attack => {
                                if let Some(attack_stats) = &unit.attack_stats {
                                    new_position.x += (attack_stats.direction.0 * (unit.move_speed) as f32 * delta_time as f32) as i32;
                                    new_position.y += (attack_stats.direction.1 * (unit.move_speed) as f32 * delta_time as f32) as i32;
                                }
                            }
                            ObjectType::Player => {
                                if current_tab == GameTab::Adventure && in_focus {
                                    let dx = key_state.d.load(Ordering::Relaxed) as i32 - key_state.a.load(Ordering::Relaxed) as i32;
                                    let dy = key_state.s.load(Ordering::Relaxed) as i32 - key_state.w.load(Ordering::Relaxed) as i32;

                                    new_position.x += dx * distance as i32;
                                    new_position.y += dy * distance as i32;
                                }
                            }
                            ObjectType::Enemy => {
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
                            ObjectType::Collectable => {
                                let direction_vec = player_position.sub(old_position);
                                let length_squared = direction_vec.x as i64 * direction_vec.x as i64 + direction_vec.y as i64 * direction_vec.y as i64;
                                let pickup_radius_squared = pickup_radius as i64 * pickup_radius as i64;

                                if length_squared >= pickup_radius_squared {
                                    local_buffer.push((unit.id, old_position, new_position));
                                    continue;
                                }

                                if length_squared > 0 {
                                    let abs_length = int_sqrt_64(length_squared);
                                    if abs_length > 0 {
                                        let base_speed = distance as i64; // Normal move speed

                                        let scale_factor = pickup_radius_squared / length_squared;
                                        let mut scaled_speed = base_speed * scale_factor;

                                        // ✅ Cap at 16x speed when within 1/4 radius
                                        if length_squared <= pickup_radius_squared / 16 {
                                            scaled_speed = scaled_speed.min(base_speed * 16);
                                        }

                                        // ✅ Ensure movement does not exceed direction_vec
                                        let move_x = ((direction_vec.x as i64 * scaled_speed) / abs_length) as i32;
                                        let move_y = ((direction_vec.y as i64 * scaled_speed) / abs_length) as i32;

                                        new_position.x += move_x;
                                        new_position.y += move_y;
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

        let attacks_to_spawn = handle_collision(&mut unit_movements, Arc::clone(&self.game_data), delta_time);
        move_units_batched(&unit_movements, &self.game_data, player_id);

        for (attack_name, unit_position, unit_id) in attacks_to_spawn {
            spawn_attack(Arc::clone(&self.game_data), attack_name.clone(), unit_position, Some(unit_id), true);
            let mut units = acquire_lock_mut(&self.game_data.units, "");
            if let Some(Some(unit)) = units.get_mut(unit_id as usize) {
                if let Some(attack_stats) = get_modified_attack(&unit.upgrades, attack_name.clone()).attack_stats {
                    unit.attack_cooldowns.insert(attack_name.clone(), attack_stats.cooldown);
                }
            }
        }
    }

    pub fn start_game(mut self) {
        loop {
            if !self.game_data.game_loop_active.load(Ordering::Relaxed) {
                sleep(Duration::from_millis(10));
                self.updated_at = Instant::now();
                continue;
            }

            let now = Instant::now();
            self.update();

            let elapsed = now.elapsed().as_millis() as u64;
            if GAME_RATE > elapsed {
                let sleep_milli = max(GAME_RATE - elapsed, 10);
                sleep(Duration::from_millis(sleep_milli));
            } else {
                sleep(Duration::from_millis(10));
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