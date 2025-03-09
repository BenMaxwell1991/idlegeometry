use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::constants::GAME_RATE;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, GAME_IN_FOCUS, KEY_STATE, RESOURCES, UNITS};
use crate::game::units::unit_type::UnitType;
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
        self.handle_animations(delta_time);
        self.handle_movement(delta_time);
        self.updated_at = Instant::now();
    }

    fn handle_animations(&mut self, delta_time: f64) {
        if let Some(units) = self.game_data.get_field(UNITS) {
            let mut updated_units = units.clone();
            for unit in updated_units.iter_mut() {
                unit.animation.animation_frame = (unit.animation.animation_frame + delta_time as f32 / unit.animation.animation_length.as_secs_f32()).fract();
            }
            self.game_data.update_field(UNITS, |unit| { *unit = updated_units });
        }
    }

    fn handle_movement(&mut self, delta_time: f64) {
        let current_tab = self.game_data.get_field(CURRENT_TAB).unwrap_or(NullGameTab);
        let in_focus = self.game_data.get_field(GAME_IN_FOCUS).unwrap_or(false);

        if in_focus && current_tab == GameTab::Adventure {
            if let Some(units) = self.game_data.get_field(UNITS) {
                let mut updated_units = units.clone();

                for unit in updated_units.iter_mut() {
                    if unit.unit_type == UnitType::Player {
                        if let Some(key_state) = self.game_data.get_field(KEY_STATE) {
                            let distance = unit.stats.iter().find(|stat| stat.name == "movement_speed").unwrap().amount.to_f32() * delta_time as f32;
                            if key_state.w.load(Ordering::SeqCst) { unit.position.y -= distance; }
                            if key_state.s.load(Ordering::SeqCst) { unit.position.y += distance; }
                            if key_state.a.load(Ordering::SeqCst) { unit.position.x -= distance; }
                            if key_state.d.load(Ordering::SeqCst) { unit.position.x += distance; }
                        }
                    }
                }
                self.game_data.update_field(UNITS, |unit| { *unit = updated_units });
            }
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
