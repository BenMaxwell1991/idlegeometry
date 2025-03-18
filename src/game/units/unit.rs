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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub unit_shape: UnitShape,
    pub move_speed: i32,
    pub health_max: f32,
    pub health_current: f32,
    pub animation: Animation,
    pub attacks: Vec<AttackName>,
    pub upgrades: Vec<Upgrade>,
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
            attacks: Vec::new(),
            upgrades: Vec::new(),
        }
    }
    pub fn apply_damage(&mut self, damage: f64) -> bool {
        self.health_current -= damage as f32;
        self.health_current <= 0.0
    }
}

pub fn add_units(units: Vec<Unit>, positions: Vec<Pos2FixedPoint>, game_data: &GameData) {
    let mut game_units = game_data.units.write().unwrap();
    let mut unit_positions = game_data.unit_positions.write().unwrap();
    let mut empty_indexes = game_data.empty_unit_indexes.write().unwrap();
    let mut spatial_grid = game_data.spatial_hash_grid.write().unwrap();

    for (mut unit, position) in units.into_iter().zip(positions.into_iter()) {
        let unit_id = if let Some(reuse_index) = empty_indexes.pop() {
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

pub fn remove_units(unit_ids: Vec<u32>, game_data: &GameData) {
    let mut game_units = game_data.units.write().unwrap();
    let mut unit_positions = game_data.unit_positions.write().unwrap();
    let mut empty_indexes = game_data.empty_unit_indexes.write().unwrap();
    let mut spatial_grid = game_data.spatial_hash_grid.write().unwrap();

    for &unit_id in &unit_ids {
        if let Some(unit) = game_units.get_mut(unit_id as usize) {
            if let Some(unit) = unit.take() {
                spatial_grid.remove_unit(&unit_id, unit_positions[unit_id as usize]);
                unit_positions[unit_id as usize] = Pos2FixedPoint::new(INVALID_POSITION, INVALID_POSITION);
                empty_indexes.push(unit_id);
            }
        }
    }
}

pub fn move_units_batched(unit_positions_updates: &[(u32, Pos2FixedPoint, Pos2FixedPoint)], game_data: &GameData, delta_time: f64) {
    let mut game_units = game_data.units.write().unwrap();
    let mut unit_positions = game_data.unit_positions.write().unwrap();
    let mut spatial_grid = game_data.spatial_hash_grid.write().unwrap();
    let mut camera_state = game_data.camera_state.write().unwrap();

    unit_positions.clear();
    let mut new_positions: Vec<Pos2FixedPoint> = Vec::with_capacity(unit_positions_updates.len());
    new_positions = unit_positions_updates.par_iter().map(|&(_, _, new_pos)| new_pos).collect();
    swap(&mut *unit_positions, &mut new_positions);
    spatial_grid.update_units_position_in_grid(unit_positions_updates);

    let pos = game_units.iter()
        .filter_map(|unit| unit.as_ref())
        .find_map(|unit| {
            if unit.unit_type == UnitType::Player {
                Some(unit_positions[unit.id as usize])
            } else {
                None
            }
        }).unwrap_or(Pos2FixedPoint::default());

    camera_state.set_target(pos);
    camera_state.move_to_target();
}

pub fn apply_upgrade(unit: &mut Unit, upgrade_type: UpgradeType) {
    if let Some(existing_upgrade) = unit.upgrades.iter_mut().find(|u| u.upgrade_type == upgrade_type) {
        existing_upgrade.level += 1;
    } else {
        unit.upgrades.push(Upgrade { upgrade_type, level: 1 });
    }

    println!("Upgrade Applied: {:?}", unit.upgrades);
}

