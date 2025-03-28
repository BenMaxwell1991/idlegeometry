use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttackStats {
    pub name: AttackName,
    pub damage: f64,
    pub cooldown: f32,
    pub direction: (f32, f32),
    pub speed: i32,
    pub lifetime: f32,
    pub elapsed: f32,
    pub damage_point: f32,
    pub damage_duration: f32,
    pub enabled: bool,
    pub hit_count: u32,
    pub max_targets: u32,
    pub units_hit: Vec<u32>,
    pub cast_sounds: Vec<String>,
    pub use_parent_shape: bool,
    pub proximity_attack: bool,

    // Multi-projectile settings:
    pub projectile_count: u32,
    pub spread_angle: f32,
    pub starting_angle: f32,
    pub burst_count: u32,
    pub burst_delay: f32,
    pub initial_burst: bool,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Copy)]
pub enum AttackName {
    Proximity,
    Swipe,
    FireBolt,
    LightningBolt,
}

impl Default for AttackStats {
    fn default() -> Self {
        AttackStats {
            name: AttackName::FireBolt,
            damage: 1.0,
            cooldown: 8.0,
            direction: (1.0, 0.0),
            speed: 300 * FIXED_POINT_SCALE,
            lifetime: 2.0,
            elapsed: 0.0,
            damage_point: 0.0,
            damage_duration: 3.0,
            enabled: false,
            hit_count: 0,
            max_targets: 1,
            units_hit: Vec::new(),
            cast_sounds: vec![],
            use_parent_shape: false,
            proximity_attack: false,

            // Multi-projectile defaults:
            projectile_count: 1,  // Single shot by default
            spread_angle: 0.0,    // No spread
            starting_angle: 0.0,  // Directly forward
            burst_count: 1,       // Fires once per activation
            burst_delay: 0.0,     // No delay needed
            initial_burst: true,     // Keep track of bursts
        }
    }
}