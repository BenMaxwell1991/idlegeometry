use std::cmp::max;
use crate::game::data::game_data::GameData;
use crate::game::data::resource_cost::ResourceAmount;
use crate::helper::lock_helper::acquire_lock_mut;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::game::constants::GAME_RATE;

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
            if !object.unlocked {
                updated_last_produced_times.push(Instant::now());
                continue;
            }

            let duration_secs = object.production_duration as f64 / 1000.0;
            let elapsed = object.last_produced.elapsed().as_secs_f64() + delta_time;

            let ticks = (elapsed / duration_secs).floor() as u64;
            let mut produced_any = false;

            for _ in 0..ticks {
                if Self::can_afford(resources, &object.production_cost) {
                    Self::pay_cost(resources, &object.production_cost);
                    Self::add_production(resources, &object.production_amount);
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

    fn can_afford(resources: &ResourceAmount, cost: &ResourceAmount) -> bool {
        macro_rules! afford_field {
            ($field:ident) => {
                cost.$field.map_or(true, |required| {
                    resources.$field.unwrap_or(0.0) >= required
                })
            };
        }

        afford_field!(gold)
            && afford_field!(ruby)
            && afford_field!(gemstone)
            && afford_field!(experience)
            && afford_field!(fire)
            && afford_field!(food)
    }

    fn pay_cost(resources: &mut ResourceAmount, cost: &ResourceAmount) {
        macro_rules! pay_field {
            ($field:ident) => {
                if let Some(amount) = cost.$field {
                    *resources.$field.get_or_insert(0.0) -= amount;
                }
            };
        }

        pay_field!(gold);
        pay_field!(ruby);
        pay_field!(gemstone);
        pay_field!(experience);
        pay_field!(fire);
        pay_field!(food);
    }

    pub fn add_production(resources: &mut ResourceAmount, production: &ResourceAmount) {
        macro_rules! add_field {
            ($field:ident) => {
                if let Some(amount) = production.$field {
                    *resources.$field.get_or_insert(0.0) += amount;
                }
            };
        }

        add_field!(gold);
        add_field!(ruby);
        add_field!(gemstone);
        add_field!(experience);
        add_field!(fire);
        add_field!(food);
    }
}
