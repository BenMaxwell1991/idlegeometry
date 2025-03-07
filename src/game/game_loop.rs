use crate::game::constants::GAME_RATE;
use crate::game::game_data::GameData;
use crate::resources::resource::Resource;
use crossbeam::channel::Sender;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use egui::{Key, Pos2};

pub struct GameLoop {
    pub game_data: Arc<GameData>,
    pub updated_at: Instant,
}

impl GameLoop {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self {
            game_data,
            updated_at: Instant::now() ,
        }
    }

    pub fn update(&mut self) {
        let delta_time = Instant::now().duration_since(self.updated_at).as_secs_f64();

        // Resources
        self.game_data.update_field::<Vec<Resource>>("resources", |resources| {
            for resource in resources.iter_mut() {
                resource.update(delta_time);
            }
        });

        // Movement
        self.handle_movement(delta_time);

        self.updated_at = Instant::now();
    }

    fn handle_movement(&self, delta_time: f64) {
        let movement_speed = 400.0;

        let mut new_pos = self.game_data.get_field::<Pos2>("player_position").unwrap_or(Pos2::new(100.0, 100.0));
        let input = self.game_data.get_field::<Vec<Key>>("pressed_keys").unwrap_or_default();

        if input.contains(&Key::W) { new_pos.y -= movement_speed * delta_time as f32; }
        if input.contains(&Key::S) { new_pos.y += movement_speed * delta_time as f32; }
        if input.contains(&Key::A) { new_pos.x -= movement_speed * delta_time as f32; }
        if input.contains(&Key::D) { new_pos.x += movement_speed * delta_time as f32; }

        self.game_data.update_field::<Pos2>("player_position", |pos| {
            *pos = new_pos;
        });
    }

    pub fn start_game(mut self, sender: Sender<()>) {
        let update_rate = Duration::from_millis(GAME_RATE);

        loop {
            let now = Instant::now();
            self.update();

            if sender.send(()).is_err() {
                eprintln!("Error: Failed to send game loop tick.");
            }

            let elapsed = now.elapsed();
            if elapsed < update_rate {
                thread::sleep(update_rate - elapsed);
            }
        }
    }
}
