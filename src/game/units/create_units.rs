use crate::game::resources::resource::{DEFAULT_HEALTH, DEFAULT_MANA, DEFAULT_MOVE_SPEED};
use crate::game::units::animation::Animation;
use crate::game::units::unit::Unit;
use crate::game::units::unit_type::UnitType;
use egui::Pos2;
use std::time::Duration;

pub fn create_enemy_at_point(handle: &str, pos2: Pos2) -> Unit {
    let stats = vec![DEFAULT_MOVE_SPEED.clone(), DEFAULT_HEALTH.clone(), DEFAULT_MANA.clone()];
    let animation = Animation::new(handle, Duration::from_secs(1));
    Unit::new(UnitType::Enemy, pos2, stats, animation)
}