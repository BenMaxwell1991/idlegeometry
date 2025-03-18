use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::units::animation::Animation;
use crate::game::units::unit::Unit;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::upgrades::UpgradeType;
use crate::ui::asset::sprite::sprite_sheet::{BABY_GREEN_DRAGON, SLASH_ATTACK};
use crate::ui::sound::music_player::{ATTACK_SWIPE_01, ATTACK_SWIPE_02};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Attack {
    pub id: u32,
    pub origin_unit_id: Option<u32>,
    pub name: AttackName,
    pub attack_shape: UnitShape,
    pub damage: f64,
    pub range: f32,
    pub cooldown: f32,
    pub direction: (f32, f32),
    pub speed: i32,
    pub area: f32,
    pub animation: Animation,
    pub attack_origin: Pos2FixedPoint,

    // Attack timings
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

impl Attack {
    pub fn get_modified_attack(unit: &Unit, attack_name: AttackName) -> Self {
        let mut attack = Attack::get_basic_attack(attack_name);

        for upgrade in &unit.upgrades {
            match upgrade.upgrade_type {
                UpgradeType::IncreaseDamage => {
                    attack.damage += 10.0 * upgrade.level as f64;
                }
                UpgradeType::DecreaseCooldown => {
                    attack.cooldown *= 1.0 - (0.05 * upgrade.level as f32);
                }
                UpgradeType::IncreaseAOE => {
                    attack.area += 2.0 * FIXED_POINT_SCALE as f32 * upgrade.level as f32;
                }
                UpgradeType::IncreaseRange => {
                    attack.range += 10.0 * FIXED_POINT_SCALE as f32 * upgrade.level as f32;
                }
                UpgradeType::IncreaseSpeed => {
                    attack.speed += 1 * FIXED_POINT_SCALE * upgrade.level as i32;
                }
            }
        }

        attack
    }

    pub fn get_basic_attack(name: AttackName) -> Self {
        let animation = match name {
            AttackName::Swipe => Animation::new(SLASH_ATTACK, Duration::from_millis(400), (200, 70)),
            AttackName::Fireball => Animation::new(BABY_GREEN_DRAGON, Duration::from_millis(300), (70, 70)),
        };

        match name {
            AttackName::Swipe => Self {
                id: u32::MAX,
                origin_unit_id: None,
                name: AttackName::Swipe,
                attack_shape: UnitShape::new(200 * FIXED_POINT_SCALE, 70 * FIXED_POINT_SCALE),
                damage: 1.0,
                range: 50.0,
                cooldown: 0.6,
                direction: (0.0, 1.0),
                speed: 0 * FIXED_POINT_SCALE,
                area: 2000.0,
                animation,
                attack_origin: Pos2FixedPoint::new(0, 0),
                lifetime: 0.4,
                elapsed: 0.0,
                damage_point: 0.0,
                damage_duration: 0.0,
                enabled: false,
                hit_count: 0,
                max_targets: u32::MAX,
                units_hit: Vec::new(),
                cast_sounds: vec![ATTACK_SWIPE_01.to_string(), ATTACK_SWIPE_02.to_string()],
            },
            AttackName::Fireball => Self {
                id: u32::MAX,
                origin_unit_id: None,
                name: AttackName::Fireball,
                attack_shape: UnitShape::new(20 * FIXED_POINT_SCALE, 20 * FIXED_POINT_SCALE),
                damage: 50.0,
                range: 200.0,
                cooldown: 5.0,
                direction: (1.0, 0.0),
                speed: 2 * FIXED_POINT_SCALE,
                area: 30.0,
                animation,
                attack_origin: Pos2FixedPoint::new(0, 0),
                lifetime: 1.0,
                elapsed: 0.0,
                damage_point: 0.0,
                damage_duration: 1.0,
                enabled: false,
                hit_count: 0,
                max_targets: 1,
                units_hit: Vec::new(),
                cast_sounds: Vec::new(),
            },
        }
    }
}
