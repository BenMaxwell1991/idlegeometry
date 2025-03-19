use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::{Pos2FixedPoint, INVALID_POSITION};
use crate::game::units::animation::Animation;
use crate::game::units::attack::AttackName;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use crate::game::units::upgrades::{Upgrade, UpgradeType};
use rayon::iter::*;
use serde::{Deserialize, Serialize};
use std::mem::swap;
use std::sync::Arc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::game::units::loot::Loot;
use crate::game::units::on_death::OnDeath;
use crate::game::units::unit_defaults::collectable_01_basic_monster;
use crate::helper::lock_helper::acquire_lock_mut;
use crate::ui::sound::music_player::play_sound;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub unit_shape: UnitShape,
    pub move_speed: i32,
    pub health_max: f32,
    pub health_current: f32,
    pub animation: Animation,
    pub attack_cooldowns: FxHashMap<AttackName, f32>,
    pub upgrades: Vec<Upgrade>,
    pub pickup_radius: Option<i32>,
    pub loot: Option<Loot>,
    pub on_death: OnDeath,
}

impl Unit {
    pub fn new(unit_type: UnitType, unit_shape: UnitShape, move_speed: i32, health_max: f32, health_current: f32, animation: Animation) -> Self {
        Self {
            id: u32::MAX,
            unit_type,
            unit_shape,
            move_speed,
            health_max,
            health_current,
            animation,
            attack_cooldowns: FxHashMap::default(),
            upgrades: Vec::new(),
            pickup_radius: None,
            loot: None,
            on_death: OnDeath::default(),
        }
    }
    pub fn apply_damage(&mut self, damage: f64) -> bool {
        self.health_current -= damage as f32;
        self.health_current <= 0.0
    }
}

pub fn add_units(units: Vec<Unit>, positions: Vec<Pos2FixedPoint>, game_data: &GameData) {
    let mut game_units = acquire_lock_mut(&game_data.units, "Failed to acquire game_units lock");
    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "Failed to acquire unit_positions lock");
    let mut empty_indexes = acquire_lock_mut(&game_data.empty_unit_indexes, "Failed to acquire empty_unit_indexes lock");
    let mut spatial_grid = acquire_lock_mut(&game_data.spatial_hash_grid, "Failed to acquire spatial_grid lock");

    for (mut unit, position) in units.into_iter().zip(positions.into_iter()) {
        let unit_id = if let Some(reuse_index) = empty_indexes.pop() {
            unit.id = reuse_index;
            game_units[reuse_index as usize] = Some(unit.clone());
            unit_positions[reuse_index as usize] = position;
            reuse_index
        } else {
            let new_index = game_units.len() as u32;
            unit.id = new_index;
            game_units.push(Some(unit.clone()));
            unit_positions.push(position);
            new_index
        };

        spatial_grid.insert_unit(unit_id, position);
    }
}

pub fn remove_units(unit_ids: Vec<u32>, game_data: Arc<GameData>) -> Vec<(Unit, Pos2FixedPoint)> {
    let mut game_units = acquire_lock_mut(&game_data.units, "Failed to acquire game_units lock");
    let mut unit_positions = acquire_lock_mut(&game_data.unit_positions, "Failed to acquire unit_positions lock");
    let mut spatial_grid = acquire_lock_mut(&game_data.spatial_hash_grid, "Failed to acquire spatial_grid lock");
    let mut empty_indexes = acquire_lock_mut(&game_data.empty_unit_indexes, "Failed to acquire empty_unit_indexes lock");

    let mut collectables_to_spawn = Vec::new();
    let mut sounds_to_play = FxHashSet::default();

    for &unit_id in &unit_ids {
        if let Some(unit) = game_units.get_mut(unit_id as usize) {
            if let Some(unit) = unit.take() {
                let position = unit_positions[unit_id as usize];

                if let Some(sound) = &unit.on_death.sound {
                    sounds_to_play.insert(sound.clone());
                }

                // If the unit is an enemy with loot, return a collectable
                if unit.unit_type == UnitType::Enemy {
                    if let Some(loot) = unit.loot {
                        let collectable = collectable_01_basic_monster(Some(loot));
                        collectables_to_spawn.push((collectable, position));
                    }
                }

                // Remove the unit itself
                spatial_grid.remove_unit(&unit_id, position);
                unit_positions[unit_id as usize] = Pos2FixedPoint::new(INVALID_POSITION, INVALID_POSITION);
                empty_indexes.push(unit_id);
            }
        }
    }

    for sound in sounds_to_play {
        play_sound(Arc::clone(&game_data), &sound.name, sound.volume);
    }

    collectables_to_spawn
}

pub fn move_units_batched(unit_positions_updates: &[(u32, Pos2FixedPoint, Pos2FixedPoint)], game_data: &GameData, delta_time: f64) {
    // let mut game_units = game_data.units.write().unwrap();
    let mut unit_positions = game_data.unit_positions.write().unwrap();
    let mut spatial_grid = game_data.spatial_hash_grid.write().unwrap();
    // let mut camera_state = game_data.camera_state.write().unwrap();

    unit_positions.clear();
    let mut new_positions: Vec<Pos2FixedPoint> = Vec::with_capacity(unit_positions_updates.len());
    new_positions = unit_positions_updates.par_iter().map(|&(_, _, new_pos)| new_pos).collect();
    swap(&mut *unit_positions, &mut new_positions);
    spatial_grid.update_units_position_in_grid(unit_positions_updates);

    // let pos = game_units.iter()
    //     .filter_map(|unit| unit.as_ref())
    //     .find_map(|unit| {
    //         if unit.unit_type == UnitType::Player {
    //             Some(unit_positions[unit.id as usize])
    //         } else {
    //             None
    //         }
    //     }).unwrap_or(Pos2FixedPoint::default());
    //
    // camera_state.set_target(pos);
    // camera_state.move_to_target();
}

pub fn apply_upgrade(unit: &mut Unit, upgrade_type: UpgradeType) {
    if let Some(existing_upgrade) = unit.upgrades.iter_mut().find(|u| u.upgrade_type == upgrade_type) {
        existing_upgrade.level += 1;
    } else {
        unit.upgrades.push(Upgrade { upgrade_type, level: 1 });
    }

    println!("Upgrade Applied: {:?}", unit.upgrades);
}

