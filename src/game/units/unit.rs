use crate::game::data::game_data::GameData;
use crate::game::resources::resource::Resource;
use crate::game::units::animation::Animation;
use crate::game::units::attack::Attack;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use egui::Pos2;
use rayon::iter::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem::swap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub unit_shape: UnitShape,
    pub stats: HashMap<String, Resource>,
    pub move_speed: f32,
    pub animation: Animation,
    pub attacks: Vec<Attack>,
}

impl Unit {
    pub fn new(unit_type: UnitType, unit_shape: UnitShape, move_speed: f32, stats: HashMap<String, Resource>, animation: Animation) -> Self {
        Self {
            id: u32::MAX,
            unit_type,
            unit_shape,
            move_speed,
            stats,
            animation,
            attacks: Vec::new(),
        }
    }
}

pub fn add_units(units: Vec<Unit>, positions: Vec<Pos2>, game_data: &GameData) {
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
                unit_positions[unit_id as usize] = Pos2::new(f32::NAN, f32::NAN);
                empty_indexes.push(unit_id);
            }
        }
    }
}

pub fn move_units_batched(unit_positions_updates: &[(u32, Pos2, Pos2)], game_data: &GameData) {
    let mut unit_positions = game_data.unit_positions.write().unwrap();
    let mut spatial_grid = game_data.spatial_hash_grid.write().unwrap();

    unit_positions.clear();
    let mut new_positions: Vec<Pos2> = Vec::with_capacity(unit_positions_updates.len());

    new_positions = unit_positions_updates.par_iter().map(|&(_, _, new_pos)| new_pos).collect();
    swap(&mut *unit_positions, &mut new_positions);

    spatial_grid.update_units_position_in_grid(unit_positions_updates);
}


fn hash_position(pos: Pos2, cell_size: f32) -> (i32, i32) {
    ((pos.x / cell_size) as i32, (pos.y / cell_size) as i32)
}
