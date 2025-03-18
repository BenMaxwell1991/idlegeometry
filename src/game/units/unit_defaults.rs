use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use crate::game::units::animation::Animation;
use crate::game::units::loot::Loot;
use crate::game::units::unit::Unit;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use crate::ui::asset::sprite::sprite_sheet::{ADULT_WHITE_DRAGON, AQUA_DRAKE, YOUNG_RED_DRAGON};
use std::time::Duration;
use crate::game::units::on_death;
use crate::game::units::on_death::OnDeath;
use crate::game::units::sound::Sound;
use crate::ui::sound::music_player::{MONSTER_DEATH_01, SELL_GOLD};

pub fn create_01_baby_dragon() -> Unit {
    let animation = Animation::new(YOUNG_RED_DRAGON, Duration::from_secs(1), (25, 25));

    let mut unit = Unit::new(
        UnitType::Enemy,
        UnitShape::new(25 * FIXED_POINT_SCALE, 25 * FIXED_POINT_SCALE),
        25 * FIXED_POINT_SCALE,
        1.0,
        1.0,
        animation
    );

    let loot = Loot {
        gold: 1.0,
        exp: 1.0,
    };


    unit.on_death = OnDeath {
        sound: Some(Sound::death_01_default()),
        animation: None,
    };

    unit.loot = Some(loot);
    unit
}

pub fn create_02_aqua_drake() -> Unit {
    let animation = Animation::new(AQUA_DRAKE, Duration::from_secs(1), (20, 20));

    let mut unit = Unit::new(
        UnitType::Enemy,
        UnitShape::new(15 * FIXED_POINT_SCALE, 15 * FIXED_POINT_SCALE),
        40 * FIXED_POINT_SCALE,
        10.0,
        10.0,
        animation
    );

    let loot = Loot {
        gold: 5.0,
        exp: 10.0,
    };

    unit.on_death = OnDeath {
        sound: Some(Sound::death_01_default()),
        animation: None,
    };

    unit.loot = Some(loot);
    unit
}

pub fn create_03_adult_white_dragon() -> Unit {
    let animation = Animation::new(ADULT_WHITE_DRAGON, Duration::from_secs(1), (45, 45));

    let mut unit = Unit::new(
        UnitType::Enemy,
        UnitShape::new(40 * FIXED_POINT_SCALE, 40 * FIXED_POINT_SCALE),
        30 * FIXED_POINT_SCALE,
        100.0,
        100.0,
        animation
    );

    let loot = Loot {
        gold: 25.0,
        exp: 75.0,
    };

    unit.on_death = OnDeath {
        sound: Some(Sound::death_01_default()),
        animation: None,
    };

    unit.loot = Some(loot);
    unit
}

pub fn collectable_01_basic_monster(loot: Option<Loot>) -> Unit {
    let animation = Animation::new(ADULT_WHITE_DRAGON, Duration::from_secs(1), (45, 45));

    let mut collectable = Unit::new(
        UnitType::Collectable,
        UnitShape::new(50 * FIXED_POINT_SCALE, 50 * FIXED_POINT_SCALE),
        30 * FIXED_POINT_SCALE,
        100.0,
        100.0,
        animation
    );

    let sound = Sound {
        name: SELL_GOLD.to_string(),
        volume: 1.0,
    };

    collectable.on_death = OnDeath {
        sound: Some(sound),
        animation: None,
    };

    collectable.loot = loot;
    collectable
}