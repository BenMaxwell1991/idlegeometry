use crate::game::maths::pos_2::Pos2FixedPoint;
use egui::Color32;
use std::time::Instant;

#[derive(Clone)]
pub struct DamageNumber {
    pub position: Pos2FixedPoint,
    pub value: f64,
    pub spawn_time: Instant,
    pub colour: Color32,
}