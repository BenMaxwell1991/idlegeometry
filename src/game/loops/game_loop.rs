use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::constants::GAME_RATE;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CAMERA_STATE, CURRENT_TAB, GAME_IN_FOCUS, KEY_STATE, RESOURCES};
use crate::game::loops::key_state::KeyState;
use crate::game::units::unit_type::UnitType;
use egui::{vec2, Pos2};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::cmp::min;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

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

        self.game_data.update_field(RESOURCES, |resources| {
            for resource in resources.iter_mut() {
                resource.update(delta_time);
            }
        });

        self.handle_animations(delta_time);
        self.handle_movement(delta_time);
        self.handle_attacks(delta_time);

        self.game_data.update_field(CAMERA_STATE, |camera| {
            camera.update_position(delta_time, 7.0 * camera.zoom);
        });

        self.updated_at = Instant::now();
    }

    fn handle_attacks(&self, delta_time: f64) {

    }

    fn handle_animations(&self, delta_time: f64) {
        self.game_data.units.write().unwrap().par_iter_mut().for_each(|unit| {
            unit.animation.animation_frame = (unit.animation.animation_frame + delta_time as f32 / unit.animation.animation_length.as_secs_f32()).fract();
        });
    }

    fn handle_movement(&self, delta_time: f64) {
        let current_tab = self.game_data.get_field(CURRENT_TAB).unwrap_or(NullGameTab);
        let in_focus = self.game_data.get_field(GAME_IN_FOCUS).unwrap_or(false);
        let key_state = self.game_data.get_field(KEY_STATE).unwrap_or(Arc::new(KeyState::new()));

        if in_focus && current_tab == GameTab::Adventure {
            if let Some(player) = self.game_data.units.write().unwrap().par_iter_mut().find_first(|u| u.unit_type == UnitType::Player) {
                let movement_speed = player.stats.iter()
                    .find(|stat| stat.name == "movement_speed")
                    .unwrap()
                    .amount.to_f32();
                let distance = movement_speed * delta_time as f32;

                if key_state.w.load(Ordering::SeqCst) { player.position.y -= distance; }
                if key_state.s.load(Ordering::SeqCst) { player.position.y += distance; }
                if key_state.a.load(Ordering::SeqCst) { player.position.x -= distance; }
                if key_state.d.load(Ordering::SeqCst) { player.position.x += distance; }

                self.update_camera_position(player.position);
            }
        }
    }

    fn update_camera_position(&self, player_position: Pos2) {
        let screen_size = self.game_data.graphic_window_size.read().unwrap().unwrap_or((vec2(0.0, 0.0)));

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
        let update_rate = Duration::from_millis(GAME_RATE);

        loop {
            let now = Instant::now();

            self.update();

            let elapsed = now.elapsed();
            if elapsed < update_rate {
                thread::sleep(min(update_rate - elapsed, Duration::from_millis(GAME_RATE)));
            }
        }
    }
}
