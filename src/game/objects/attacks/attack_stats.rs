use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttackStats {
    pub name: AttackName,
    pub damage: f64,
    pub range: f32,
    pub cooldown: f32,
    pub direction: (f32, f32),
    pub speed: i32,
    pub area: f32,
    pub lifetime: f32,
    pub elapsed: f32,
    pub damage_point: f32,
    pub damage_duration: f32,
    pub enabled: bool,
    pub hit_count: u32,
    pub max_targets: u32,
    pub units_hit: Vec<u32>,
    pub cast_sounds: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum AttackName {
    Swipe,
    Fireball
}