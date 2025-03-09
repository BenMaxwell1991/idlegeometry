use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub struct Animation {
    pub sprite_key: String,
    pub animation_length: Duration,
    pub animation_frame: f32,
}

impl Animation {
    pub fn new(sprite_key: &str, animation_length: Duration) -> Self {
        Self {
            sprite_key: sprite_key.to_string(),
            animation_length,
            animation_frame: rand::rng().random_range(0.0..=1.0),
        }
    }
}