use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use crate::game::objects::animation::Animation;
use crate::game::objects::game_object::GameObject;
use crate::game::objects::loot::Loot;
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use crate::game::objects::on_death::OnDeath;
use crate::game::objects::sound::Sound;
use crate::ui::asset::sprite::sprite_sheet::{ADULT_WHITE_DRAGON, AQUA_DRAKE, TREASURE, YOUNG_RED_DRAGON};
use std::time::Duration;
use crate::ui::sound::kira_audio::SOUND_01;

pub fn create_01_baby_dragon() -> GameObject {
    let animation = Animation::new(YOUNG_RED_DRAGON, Duration::from_secs(2), (25, 25));

    let mut unit = GameObject::new(
        ObjectType::Enemy,
        ObjectShape::new(25 * FIXED_POINT_SCALE, 25 * FIXED_POINT_SCALE),
        25 * FIXED_POINT_SCALE,
        1.0,
        1.0,
        animation
    );

    let loot = Loot {
        gold: 1.0,
        exp: 1.0,
    };


    // unit.on_death = OnDeath {
    //     sound: Some(Sound::death_01_default()),
    //     animation: None,
    // };

    unit.loot = Some(loot);
    unit
}

pub fn create_02_aqua_drake() -> GameObject {
    let animation = Animation::new(AQUA_DRAKE, Duration::from_secs(2), (20, 20));

    let mut unit = GameObject::new(
        ObjectType::Enemy,
        ObjectShape::new(15 * FIXED_POINT_SCALE, 15 * FIXED_POINT_SCALE),
        40 * FIXED_POINT_SCALE,
        10.0,
        10.0,
        animation
    );

    let loot = Loot {
        gold: 5.0,
        exp: 10.0,
    };

    // unit.on_death = OnDeath {
    //     sound: Some(Sound::death_01_default()),
    //     animation: None,
    // };

    unit.loot = Some(loot);
    unit
}

pub fn create_03_adult_white_dragon() -> GameObject {
    let animation = Animation::new(ADULT_WHITE_DRAGON, Duration::from_secs(2), (45, 45));

    let mut unit = GameObject::new(
        ObjectType::Enemy,
        ObjectShape::new(40 * FIXED_POINT_SCALE, 40 * FIXED_POINT_SCALE),
        30 * FIXED_POINT_SCALE,
        100.0,
        100.0,
        animation
    );

    let loot = Loot {
        gold: 25.0,
        exp: 75.0,
    };

    // unit.on_death = OnDeath {
    //     sound: Some(Sound::death_01_default()),
    //     animation: None,
    // };

    unit.loot = Some(loot);
    unit
}

pub fn collectable_01_basic_monster(loot: Option<Loot>) -> GameObject {
    let mut animation = Animation::new(TREASURE, Duration::from_secs(1), (45, 45));

    animation.fixed_frame_index = Some(0);

    let mut collectable = GameObject::new(
        ObjectType::Collectable,
        ObjectShape::new(50 * FIXED_POINT_SCALE, 50 * FIXED_POINT_SCALE),
        30 * FIXED_POINT_SCALE,
        100.0,
        100.0,
        animation
    );

    let sound = Sound {
        name: SOUND_01.to_string(),
        volume: 0.1,
    };

    collectable.on_death = OnDeath {
        sound: Some(sound),
        animation: None,
    };

    collectable.loot = loot;
    collectable
}