use rand::Rng;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Animation {
    pub tracked_unit_id: Option<u32>,
    pub fixed_frame_index: Option<usize>,
    pub sprite_key: String,
    pub animation_length: Duration,
    pub animation_frame: f32,
    pub last_damage_time: Option<Instant>,
    pub size: (u32, u32),
}

impl Animation {
    pub fn new(sprite_key: &str, animation_length: Duration, size: (u32, u32)) -> Self {
        Self {
            tracked_unit_id: None,
            fixed_frame_index: None,
            sprite_key: sprite_key.to_string(),
            animation_length,
            animation_frame: rand::rng().random_range(0.0..=1.0),
            last_damage_time: None,
            size,
        }
    }
}