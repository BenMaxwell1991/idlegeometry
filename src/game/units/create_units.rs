use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use crate::game::units::animation::Animation;
use crate::game::units::unit::Unit;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use std::time::Duration;

pub fn create_enemy_at_point(handle: &str) -> Unit {
    let animation = Animation::new(handle, Duration::from_secs(1), (20, 20));
    Unit::new(UnitType::Enemy, UnitShape::new(20 * FIXED_POINT_SCALE, 20 * FIXED_POINT_SCALE), 30 * FIXED_POINT_SCALE, 10.0, 10.0, animation)
}