use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use crate::game::units::animation::Animation;
use crate::game::units::attack::AttackName;
use crate::game::units::loot::Loot;
use crate::game::units::unit::Unit;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use crate::ui::asset::sprite::sprite_sheet::YOUNG_RED_DRAGON;
use std::time::Duration;

pub fn create_01_baby_dragon() -> Unit {
    let animation = Animation::new(YOUNG_RED_DRAGON, Duration::from_secs(1), (20, 20));

    let mut baby_dragon = Unit::new(
        UnitType::Enemy,
        UnitShape::new(20 * FIXED_POINT_SCALE, 20 * FIXED_POINT_SCALE),
        20 * FIXED_POINT_SCALE,
        1.0,
        1.0,
        animation
    );

    let loot = Loot {
        gold: 1.0,
        exp: 1.0,
    };

    baby_dragon.loot = loot;
    baby_dragon
}