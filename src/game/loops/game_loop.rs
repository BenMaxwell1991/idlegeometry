use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::constants::GAME_RATE;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CAMERA_STATE, CURRENT_TAB, GAME_IN_FOCUS, KEY_STATE, RESOURCES};
use crate::game::loops::key_state::KeyState;
use crate::game::units::unit::move_units_batched;
use crate::game::units::unit_type::UnitType;
use egui::{vec2, Pos2};
use rayon::current_num_threads;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rayon::prelude::{IndexedParallelIterator, ParallelSliceMut};
use std::cmp::max;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use crate::game::collision::detect_collision::handle_collision;

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

        // let now = Instant::now();
        self.handle_animations(delta_time);
        // println!("Animations: {}", now.elapsed().as_nanos());

        // let now = Instant::now();
        self.handle_movement(delta_time);
        // println!("Movement: {}", now.elapsed().as_nanos());

        // let now = Instant::now();
        self.handle_attacks(delta_time);
        // println!("Attacks: {}", now.elapsed().as_nanos());

        self.game_data.update_field(CAMERA_STATE, |camera| {
            camera.update_position(delta_time, 7.0 * camera.zoom);
        });
    }

    fn handle_attacks(&self, delta_time: f64) {

    }

    fn handle_animations(&self, delta_time: f64) {
        self.game_data.units.write().unwrap().par_iter_mut().for_each(|unit| {
            if let Some(unit) = unit {
                unit.animation.animation_frame = (unit.animation.animation_frame + delta_time as f32 / unit.animation.animation_length.as_secs_f32()).fract();
            }
        });
    }

    fn handle_movement(&self, delta_time: f64) {
        let current_tab = self.game_data.get_field(CURRENT_TAB).unwrap_or(NullGameTab);
        let in_focus = self.game_data.get_field(GAME_IN_FOCUS).unwrap_or(false);

        let key_state = self.game_data.get_field(KEY_STATE).unwrap_or(Arc::new(KeyState::new()));

        let mut game_units = match self.game_data.units.write() {
            Ok(gu) => gu,
            Err(_) => return,
        };

        let mut unit_positions = match self.game_data.unit_positions.read() {
            Ok(up) => up,
            Err(_) => return,
        };

        let game_units_len = game_units.len();

        let player_position = game_units.iter()
            .filter_map(|unit| unit.as_ref())
            .find_map(|unit| {
                if unit.unit_type == UnitType::Player {
                    Some(unit_positions[unit.id as usize])
                } else {
                    None
                }
            }).unwrap();

        let num_threads = current_num_threads();
        let estimated_per_thread = (game_units_len / num_threads).max(1);

        let mut unit_movements: Vec<(u32, Pos2, Pos2)> = game_units
            .par_chunks_mut(estimated_per_thread)
            .map(|chunk| {
                let mut local_buffer = Vec::with_capacity(chunk.len() * 2);
                for (unit) in chunk.iter_mut() {
                    if let Some(unit) = unit {
                        let movement_speed = unit.move_speed;
                        let distance = movement_speed * delta_time as f32;

                        let old_position = unit_positions[unit.id as usize];
                        let mut new_position = old_position;

                        match unit.unit_type {
                            UnitType::Player => {
                                if current_tab == GameTab::Adventure && in_focus {
                                    let dx = (key_state.d.load(Ordering::Relaxed) as i32  - key_state.a.load(Ordering::Relaxed) as i32) as f32;
                                    let dy = (key_state.s.load(Ordering::Relaxed) as i32 - key_state.w.load(Ordering::Relaxed) as i32) as f32;

                                    new_position.x += dx * distance;
                                    new_position.y += dy * distance;
                                }
                            }
                            UnitType::Enemy => {
                                let direction_vec = player_position - old_position;
                                let length_squared = direction_vec.x * direction_vec.x + direction_vec.y * direction_vec.y;
                                if length_squared > 0.0 {
                                    let inv_length = length_squared.sqrt().recip();
                                    new_position.x += direction_vec.x * inv_length * distance;
                                    new_position.y += direction_vec.y * inv_length * distance;
                                }
                            }
                        };

                        local_buffer.push((unit.id, old_position, new_position));
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
        move_units_batched(&unit_movements, &self.game_data);
    }

    fn update_camera_position(&self, player_position: Pos2) {
        let screen_size = self.game_data.graphic_window_size.read().unwrap().unwrap_or(vec2(0.0, 0.0));

        self.game_data.update_field(CAMERA_STATE, |camera| {
            let screen_width = screen_size.x;
            let screen_height = screen_size.y;

            let box_width = screen_width * 0.4 / camera.zoom;
            let box_height = screen_height * 0.4 / camera.zoom;

            let min_x = camera.target_pos.x - box_width / 2.0;
            let max_x = camera.target_pos.x + box_width / 2.0;
            let min_y = camera.target_pos.y - box_height / 2.0;
            let max_y = camera.target_pos.y + box_height / 2.0;

            let mut target_x = camera.target_pos.x;
            let mut target_y = camera.target_pos.y;

            if player_position.x < min_x {
                target_x = player_position.x + box_width / 2.0;
            } else if player_position.x > max_x {
                target_x = player_position.x - box_width / 2.0;
            }

            if player_position.y < min_y {
                target_y = player_position.y + box_height / 2.0;
            } else if player_position.y > max_y {
                target_y = player_position.y - box_height / 2.0;
            }

            // Set target position for smooth movement
            camera.target_pos = Pos2::new(target_x, target_y);
        });
    }

    pub fn start_game(mut self) {
        loop {
            let now = Instant::now();
            self.update();
            println!("Game_Loop duration: {}", now.elapsed().as_micros());

            let elapsed = now.elapsed().as_micros() as u64;

            if GAME_RATE > elapsed {
                let sleep_micros = max(GAME_RATE - elapsed, 10);
                thread::sleep(Duration::from_micros(sleep_micros));
            } else {
                thread::sleep(Duration::from_micros(10));
            }
        }
    }
}
