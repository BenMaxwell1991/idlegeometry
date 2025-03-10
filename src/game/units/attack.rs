use egui::Pos2;
use crate::game::units::animation::Animation;
use serde::{Deserialize, Serialize};
use crate::game::serialise::pos2_serialisable::{deserialize_pos2, serialize_pos2};

#[derive(Clone, Serialize, Deserialize)]
pub struct Attack {
    pub name: String,
    pub damage: f64,
    pub range: f32,
    pub cooldown: f32,
    pub direction: f32,
    pub speed: f32,
    pub area: f32,
    pub animation: Animation,
    #[serde(serialize_with = "serialize_pos2", deserialize_with = "deserialize_pos2")]
    pub attack_origin: Pos2,
    pub enabled: bool,
}

impl Attack {
    pub fn new(name: &str, damage: f64, range: f32, cooldown: f32, direction: f32, speed: f32, area: f32, animation: Animation, attack_origin: Pos2, enabled: bool) -> Self {
        Self {
            name: name.to_string(),
            damage,
            range,
            cooldown,
            direction,
            speed,
            area,
            animation,
            attack_origin,
            enabled,
        }
    }
}