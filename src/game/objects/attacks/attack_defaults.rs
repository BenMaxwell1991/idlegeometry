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
        AttackName::Proximity => {
            None
        }
        AttackName::Swipe => {
            Some(Animation::new(SLASH_ATTACK, Duration::from_millis(300), (200, 70))
                .with_rotation_offset(0.0))
        }
        AttackName::FireBolt => {
            Some(Animation::new(BABY_GREEN_DRAGON, Duration::from_millis(300), (40, 40))
                .with_rotation_offset(90.0))
        }
        AttackName::LightningBolt => {
            Some(Animation::new(LIGHTNING_ZAP, Duration::from_millis(300), (40, 40))
                .with_rotation_offset(0.0))
        }
    };

    let shape = animation
        .as_ref()
        .map(|a| ObjectShape::new(
            FIXED_POINT_SCALE * a.size.0 as i32,
            FIXED_POINT_SCALE * a.size.1 as i32,
        ))
        .unwrap_or_else(|| ObjectShape::new(0, 0));

    let mut obj = GameObject {
        id: u32::MAX,
        object_type: ObjectType::Attack,
        object_shape: shape,
        move_speed: 0,
        health_max: 1.0,
        health_current: 1.0,
        animation,
        attack_cooldowns: Default::default(),
        upgrades: Vec::new(),
        pickup_radius: None,
        loot: None,
        on_death: OnDeath::default(),
        parent_unit_id: None,
        attack_stats: None,
    };

    let mut stats = AttackStats {
        name: attack_name.clone(),
        ..AttackStats::default()
    };

    match attack_name {
        AttackName::Proximity => {
            stats.damage = 5.0;
            stats.cooldown = 0.5;
            stats.speed = 0;
            stats.lifetime = 0.2;
            stats.direction = (1.0, 0.0);
            stats.damage_duration = 0.1;
            stats.max_targets = u32::MAX;
            stats.cast_sounds = vec![SOUND_01.to_string()];
            stats.use_parent_shape = true;
            stats.proximity_attack = true;
        }
        AttackName::Swipe => {
            stats.damage = 2.5;
            stats.cooldown = 3.0;
            stats.speed = 0;
            stats.lifetime = 0.3;
            stats.direction = (1.0, 0.0);
            stats.damage_duration = 0.1;
            stats.max_targets = u32::MAX;
            stats.cast_sounds = vec![SOUND_01.to_string()];
        }
        AttackName::FireBolt => {
            stats.damage = 4.0;
            stats.cooldown = 2.0;
            stats.speed = 200 * FIXED_POINT_SCALE;
            stats.lifetime = 3.0;
            stats.projectile_count = 1;
            stats.cast_sounds = vec![SOUND_01.to_string()];
        }
        AttackName::LightningBolt => {
            stats.damage = 0.5;
            stats.cooldown = 4.0;
            stats.speed = 400 * FIXED_POINT_SCALE;
            stats.lifetime = 2.0;
            stats.projectile_count = 60;
            stats.spread_angle = 360.0;
            stats.starting_angle = 90.0;
            stats.cast_sounds = vec![SOUND_01.to_string()];
        }
    }

    obj.move_speed = stats.speed;
    obj.attack_stats = Some(stats);
    obj
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
                }
                UpgradeType::IncreaseRange => {
                }
                UpgradeType::IncreaseSpeed => {
                    attack_stats.speed += 1 * FIXED_POINT_SCALE * upgrade.level as i32;
                }
            }
        }
    }

    attack
}