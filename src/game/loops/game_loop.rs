use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::constants::GAME_RATE;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, KEY_STATE, PLAYER_POSITION, RESOURCES};
use egui::Pos2;
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

        // Resources
        self.game_data.update_field(RESOURCES, |resources| {
            for resource in resources.iter_mut() {
                resource.update(delta_time);
            }
        });

        // Movement
        self.handle_movement(delta_time);
        self.updated_at = Instant::now();
    }

    fn handle_movement(&mut self, delta_time: f64) {
        let current_tab = self.game_data.get_field(CURRENT_TAB).unwrap_or(NullGameTab);

        if current_tab == GameTab::Geometry {
            let movement_speed = 400.0;
            let mut new_pos = self.game_data.get_field(PLAYER_POSITION).unwrap_or(Pos2::new(100.0, 100.0));

            if let Some(key_state) = self.game_data.get_field(KEY_STATE) {
                if key_state.w.load(Ordering::SeqCst) { new_pos.y -= movement_speed * delta_time as f32; }
                if key_state.s.load(Ordering::SeqCst) { new_pos.y += movement_speed * delta_time as f32; }
                if key_state.a.load(Ordering::SeqCst) { new_pos.x -= movement_speed * delta_time as f32; }
                if key_state.d.load(Ordering::SeqCst) { new_pos.x += movement_speed * delta_time as f32; }
            } else {
                println!("Cannot acquire key state")
            }

            self.game_data.update_field(PLAYER_POSITION, |pos| {
                *pos = new_pos;
            });
        }
    }

    pub fn start_game(mut self) {
        let update_rate = Duration::from_millis(GAME_RATE);

        loop {
            let now = Instant::now();

            self.update();

            let elapsed = now.elapsed();
            if elapsed < update_rate {
                thread::sleep(update_rate - elapsed);
            }
        }
    }
}
