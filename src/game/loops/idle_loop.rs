use crate::game::constants::GAME_RATE;
use crate::game::data::game_data::GameData;
use crate::helper::lock_helper::acquire_lock_mut;
use std::cmp::max;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::game::data::resource_cost::ResourceAmount;

pub struct IdleLoop {
    pub game_data: Arc<GameData>,
    pub updated_at: Instant,
}

impl IdleLoop {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self {
            game_data,
            updated_at: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.updated_at).as_secs_f64();
        self.updated_at = now;

        self.produce_resources(delta_time);
    }

    fn produce_resources(&self, delta_time: f64) {
        let mut player_data = acquire_lock_mut(&self.game_data.player_data, "player_data");

        let lair_objects_clone = player_data.lair_objects.clone();
        let resources = &mut player_data.resources_persistent;

        let mut updated_last_produced_times = vec![];

        for object in lair_objects_clone.iter() {
            if !object.active {
                updated_last_produced_times.push(Instant::now());
                continue;
            }

            let duration_secs = object.production_duration as f64 / 1000.0;
            let elapsed = object.last_produced.elapsed().as_secs_f64() + delta_time;

            let mut ticks = 0;
            if elapsed > duration_secs {
                ticks = (delta_time / duration_secs).ceil() as u64;
            }

            let mut produced_any = false;
            for _ in 0..ticks {
                if ResourceAmount::can_afford(resources, &object.production_cost) {
                    ResourceAmount::pay_cost(resources, &object.production_cost);
                    ResourceAmount::add_production(resources, &object.production_amount);
                    produced_any = true;
                } else {
                    break;
                }
            }

            updated_last_produced_times.push(if produced_any {
                Instant::now()
            } else {
                object.last_produced
            });
        }

        for (object, new_last_produced) in player_data
            .lair_objects
            .iter_mut()
            .zip(updated_last_produced_times)
        {
            object.last_produced = new_last_produced;
        }
    }

    pub fn start_idle_loop(mut self) {
        loop {
            self.updated_at = Instant::now();
            self.update();
            let elapsed = self.updated_at.elapsed().as_millis() as u64;

            if GAME_RATE > elapsed {
                let sleep_milli = max(GAME_RATE - elapsed, 20);
                sleep(Duration::from_millis(sleep_milli));
            } else {
                sleep(Duration::from_millis(20));
            }
        }
    }
}
