use crate::game::collision::spatial_hash_grid::SpatialHashGrid;
use crate::game::data::game_data::GameData;
use crate::game::resources::resource::Resource;
use crate::game::serialise::pos2_serialisable::{deserialize_pos2, serialize_pos2};
use crate::game::units::animation::Animation;
use crate::game::units::attack::Attack;
use crate::game::units::unit_map::UnitMap;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use egui::Pos2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: Uuid,
    pub unit_type: UnitType,
    pub unit_shape: UnitShape,
    #[serde(serialize_with = "serialize_pos2", deserialize_with = "deserialize_pos2")]
    pub position: Pos2,
    pub stats: HashMap<String, Resource>,
    pub move_speed: f32,
    pub animation: Animation,
    pub attacks: Vec<Attack>,
}

impl Unit {
    pub fn new(unit_type: UnitType, unit_shape: UnitShape, position: Pos2, move_speed: f32, stats: HashMap<String, Resource>, animation: Animation) -> Self {
        Self {
            id: Uuid::new_v4(),
            unit_type,
            unit_shape,
            position,
            move_speed,
            stats,
            animation,
            attacks: Vec::new(),
        }
    }
}

pub fn add_units(units: Vec<Unit>, game_data: &GameData) {
    let mut game_units = game_data.units.write().unwrap();
    let mut unit_map = game_data.unit_map.write().unwrap();
    let mut spatial_grid = game_data.spatial_hash_grid.write().unwrap();

    game_units.reserve(units.len());
    unit_map.map.reserve(units.len());

    let unit_start_index = game_units.len();

    for (i, unit) in units.into_iter().enumerate() {
        let unit_index = unit_start_index + i;
        let unit_id = unit.id;

        unit_map.map.insert(unit_id, unit_index);
        spatial_grid.insert_unit(unit_id, unit.position);
        game_units.push(unit);
    }
}

pub fn remove_units(unit_ids: Vec<Uuid>, game_data: &GameData) {
    let mut game_units = game_data.units.write().unwrap();
    let mut unit_map = game_data.unit_map.write().unwrap();
    let mut spatial_grid = game_data.spatial_hash_grid.write().unwrap();

    for unit_id in &unit_ids {
        unit_map.map.remove(unit_id);
    }
    spatial_grid.remove_units(&unit_ids);
    game_units.retain(|unit| !unit_ids.contains(&unit.id));
}

pub fn move_units_batched(
    unit_positions: &[(Uuid, Pos2, Pos2)],
    game_data: &GameData,
) {
    let mut game_units = match game_data.units.write() {
        Ok(gu) => gu,
        Err(_) => return,
    };

    let unit_map = match game_data.unit_map.read() {
        Ok(um) => um,
        Err(_) => return,
    };

    let mut spatial_grid = match game_data.spatial_hash_grid.write() {
        Ok(sg) => sg,
        Err(_) => return,
    };

    let now = Instant::now();
    for (unit_id, old_position, new_position) in unit_positions {
        if let Some(&unit_index) = unit_map.map.get(unit_id) {
            if let Some(unit) = game_units.get_mut(unit_index) {
                // spatial_grid.remove_unit(unit_id, *old_position);
                unit.position = *new_position;
                // spatial_grid.insert_unit(*unit_id, *new_position);
                spatial_grid.update_unit_position_in_grid(unit_id, *old_position, *new_position)
                // println!("Spatial Grid Size: {}", spatial_grid.grid.iter().len());
            }
        }
    }
    println!("Moved units in {} micro seconds", now.elapsed().as_micros());
}


pub fn move_unit(
    unit_id: &Uuid,
    old_position: Pos2,
    new_position: Pos2,
    game_data: &GameData, // ✅ Pass GameData reference instead of separate fields
) {
    let mut game_units = match game_data.units.write() {
        Ok(gu) => gu,
        Err(_) => return, // Handle lock failure
    };

    let unit_map = match game_data.unit_map.read() {
        Ok(um) => um,
        Err(_) => return, // Handle lock failure
    };

    let mut spatial_grid = match game_data.spatial_hash_grid.write() {
        Ok(sg) => sg,
        Err(_) => return, // Handle lock failure
    };

    if let Some(&unit_index) = unit_map.map.get(unit_id) {
        if let Some(unit) = game_units.get_mut(unit_index) {
            spatial_grid.remove_unit(unit_id, old_position);
            unit.position = new_position;
            spatial_grid.insert_unit(*unit_id, new_position);
        }
    }
}


fn hash_position(pos: Pos2, cell_size: f32) -> (i32, i32) {
    ((pos.x / cell_size) as i32, (pos.y / cell_size) as i32)
}
