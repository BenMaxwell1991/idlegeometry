use crate::game::constants::GAME_RATE;
use crate::game::game_data::GameData;
use crate::resources::resource::Resource;
use crossbeam::channel::Sender;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

pub struct GameLoop {
    pub game_data: Arc<GameData>,
}

impl GameLoop {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self { game_data }
    }

    pub fn update(&self, delta_time: f64) {
        self.game_data.update_field::<Vec<Resource>>("resources", |resources| {
            for resource in resources.iter_mut() {
                resource.update(delta_time);
            }
        });
    }

    pub fn start_game(self, sender: Sender<()>) {
        let update_rate = Duration::from_millis(GAME_RATE);
        let mut last_time = Instant::now();

        loop {
            let now = Instant::now();
            let delta_time = now.duration_since(last_time).as_secs_f64();
            last_time = now;

            self.update(delta_time);

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
