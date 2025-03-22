use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use crate::game::objects::animation::Animation;
use crate::game::objects::attacks::attack_stats::{AttackName, AttackStats};
use crate::game::objects::game_object::GameObject;
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use crate::game::objects::on_death::OnDeath;
use crate::game::objects::upgrades::{Upgrade, UpgradeType};
use crate::ui::asset::sprite::sprite_sheet::{BABY_GREEN_DRAGON, LIGHTNING_ZAP, SLASH_ATTACK};
use crate::ui::sound::kira_audio::SOUND_01;
use std::time::Duration;

pub fn get_basic_attack(attack_name: AttackName) -> GameObject {
    let animation = match attack_name {
        AttackName::Swipe => Animation::new(SLASH_ATTACK, Duration::from_millis(500), (200, 70)).with_rotation_offset(0.0),
        AttackName::FireBolt => Animation::new(BABY_GREEN_DRAGON, Duration::from_millis(300), (40, 40)).with_rotation_offset(90.0),
        AttackName::LightningBolt => Animation::new(LIGHTNING_ZAP, Duration::from_millis(300), (40, 40)).with_rotation_offset(0.0),
    };

    let mut attack_stats = AttackStats {
        name: attack_name.clone(),
        ..AttackStats::default()
    };

    match attack_name {
        AttackName::Swipe => {
            attack_stats.damage = 2.5;
            attack_stats.range = 50.0;
            attack_stats.cooldown = 3.0;
            attack_stats.speed = 0 * FIXED_POINT_SCALE;
            attack_stats.lifetime = 0.5;
            attack_stats.direction = (1.0, 0.0);
            attack_stats.damage_duration = 0.1;
            attack_stats.area = 2000.0;
            attack_stats.max_targets = u32::MAX;
            attack_stats.cast_sounds = vec![SOUND_01.to_string()];
        }
        AttackName::FireBolt => {
            attack_stats.damage = 4.0;
            attack_stats.lifetime = 3.0;
            attack_stats.range = 80.0;
            attack_stats.cooldown = 2.0;
            attack_stats.speed = 200 * FIXED_POINT_SCALE;
            attack_stats.projectile_count = 1;
            attack_stats.cast_sounds = vec![SOUND_01.to_string()];
        }
        AttackName::LightningBolt => {
            attack_stats.damage = 0.5;
            attack_stats.lifetime = 2.0;
            attack_stats.range = 80.0;
            attack_stats.cooldown = 4.0;
            attack_stats.speed = 400 * FIXED_POINT_SCALE;
            attack_stats.projectile_count = 60;
            attack_stats.spread_angle = 360.0;
            attack_stats.starting_angle = 90.0;
            attack_stats.cast_sounds = vec![SOUND_01.to_string()];
        }
    }

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