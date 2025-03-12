use crate::game::resources::resource::{DEFAULT_MOVE_SPEED, DEFAULT_STATS};
use crate::game::units::animation::Animation;
use crate::game::units::unit::Unit;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use std::time::Duration;

pub fn create_enemy_at_point(handle: &str) -> Unit {
    let animation = Animation::new(handle, Duration::from_secs(1));
    Unit::new(UnitType::Enemy, UnitShape::new(16.0, 16.0), DEFAULT_MOVE_SPEED, DEFAULT_STATS.clone(), animation)
}