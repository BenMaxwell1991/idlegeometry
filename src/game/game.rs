pub(crate) use crate::enums::gametab::GameTab;
use crate::game::constants::GAME_RATE;
use crate::game::settings::Settings;
use crate::resources::bignumber::BigNumber;
use crate::resources::resource::Resource;
use crossbeam::channel::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub resources: Vec<Resource>,
    pub settings: Settings,
    pub current_tab: GameTab,
}

impl Game {
    pub fn new() -> Self {
        Self {
            resources: vec! [
                Resource::new("Points", BigNumber::new(0.0), BigNumber::new(0.01), true),
                Resource::with_defaults("Lines"),
                Resource::with_defaults("Triangles"),
            ],
            settings: Settings::default(),
            current_tab: GameTab::Geometry,
        }
    }

    fn update(&mut self, delta_time: f64) {
        for resource in &mut self.resources {
            resource.update(delta_time);
        }
    }

    pub fn start_game(game: Arc<Mutex<Game>>, sender: Sender<()>) {
        let update_rate: Duration = Duration::from_millis(GAME_RATE);
        let mut last_time: Instant = Instant::now();

        loop {
            let now: Instant = Instant::now();
            let delta_time: f64 = now.duration_since(last_time).as_secs_f64();
            last_time = now;

            if let Ok(mut game) = game.lock() {
                game.update(delta_time);
            }

            sender.send(()).unwrap();

            // Sleep until the next frame of the game
            let elapsed: Duration = now.elapsed();
            if elapsed < update_rate {
                thread::sleep(update_rate - elapsed);
            }
        }
    }
}
