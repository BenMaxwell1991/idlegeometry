use crate::game::resources::resource::Resource;
use crate::game::serialise::pos2_serialisable::{deserialize_pos2, serialize_pos2};
use crate::game::units::animation::Animation;
use crate::game::units::unit_type::UnitType;
use egui::Pos2;
use serde::{Deserialize, Serialize};
use crate::game::units::attack::Attack;

#[derive(Clone, Serialize, Deserialize)]
pub struct Unit {
    pub unit_type: UnitType,
    #[serde(serialize_with = "serialize_pos2", deserialize_with = "deserialize_pos2")]
    pub position: Pos2,
    pub stats: Vec<Resource>,
    pub animation: Animation,
    pub attacks: Vec<Attack>,
}

impl Unit {
    pub fn new(unit_type: UnitType, position: Pos2, stats: Vec<Resource>, animation: Animation) -> Self {
        Self {
            unit_type,
            position,
            stats,
            animation,
            attacks: Vec::new(),
        }
    }
}
