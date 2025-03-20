use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use crate::game::objects::animation::Animation;
use crate::game::objects::attacks::attack_stats::{AttackName, AttackStats};
use crate::game::objects::game_object::GameObject;
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use crate::game::objects::upgrades::{Upgrade, UpgradeType};
use crate::ui::asset::sprite::sprite_sheet::{BABY_GREEN_DRAGON, SLASH_ATTACK};
use crate::ui::sound::music_player::ATTACK_SWIPE_02;
use std::time::Duration;
use crate::game::objects::on_death::OnDeath;

pub fn get_basic_attack(attack_name: AttackName) -> GameObject {
    let animation = match attack_name {
        AttackName::Swipe => Animation::new(SLASH_ATTACK, Duration::from_millis(1000), (200, 70)),
        AttackName::Fireball => Animation::new(BABY_GREEN_DRAGON, Duration::from_millis(300), (70, 70)),
    };

    let attack_stats = match attack_name {
        AttackName::Swipe => AttackStats {
            name: AttackName::Swipe,
            damage: 1.0,
            range: 50.0,
            cooldown: 1.0,
            direction: (0.0, 1.0),
            speed: 300 * FIXED_POINT_SCALE,
            area: 2000.0,
            lifetime: 1.0,
            elapsed: 0.0,
            damage_point: 0.0,
            damage_duration: 1.0,
            enabled: false,
            hit_count: 0,
            max_targets: u32::MAX,
            units_hit: Vec::new(),
            cast_sounds: vec![ATTACK_SWIPE_02.to_string()],
        },
        AttackName::Fireball => AttackStats {
            name: AttackName::Fireball,
            damage: 50.0,
            range: 200.0,
            cooldown: 5.0,
            direction: (1.0, 0.0),
            speed: 2 * FIXED_POINT_SCALE,
            area: 30.0,
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
    };

    GameObject {
        id: u32::MAX,
        object_type: ObjectType::Attack,
        object_shape: ObjectShape::new(FIXED_POINT_SCALE * animation.size.0 as i32, FIXED_POINT_SCALE * animation.size.1 as i32),
        move_speed: attack_stats.speed,
        health_max: 1.0,
        health_current: 1.0,
        animation,
        attack_cooldowns: Default::default(),
        upgrades: Vec::new(),
        pickup_radius: None,
        loot: None,
        on_death: OnDeath::default(),
        parent_unit_id: None,
        attack_stats: Some(attack_stats),
    }
}

pub fn get_modified_attack(upgrades: &Vec<Upgrade>, attack_name: AttackName) -> GameObject {
    let mut attack = get_basic_attack(attack_name);

    if let Some(attack_stats) = &mut attack.attack_stats {
        for upgrade in upgrades {
            match upgrade.upgrade_type {
                UpgradeType::IncreaseDamage => {
                    attack_stats.damage += 10.0 * upgrade.level as f64;
                }
                UpgradeType::DecreaseCooldown => {
                    attack_stats.cooldown *= 1.0 - (0.05 * upgrade.level as f32);
                }
                UpgradeType::IncreaseAOE => {
                    attack_stats.area += 2.0 * FIXED_POINT_SCALE as f32 * upgrade.level as f32;
                }
                UpgradeType::IncreaseRange => {
                    attack_stats.range += 10.0 * FIXED_POINT_SCALE as f32 * upgrade.level as f32;
                }
                UpgradeType::IncreaseSpeed => {
                    attack_stats.speed += 1 * FIXED_POINT_SCALE * upgrade.level as i32;
                }
            }
        }
    }

    attack
}