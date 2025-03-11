use egui::Pos2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

const CELL_SIZE: f32 = 20.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct SpatialHashGrid {
    pub grid: HashMap<(i32, i32), Vec<Uuid>>,
}

impl SpatialHashGrid {
    pub fn new() -> Self {
        Self { grid: HashMap::new() }
    }

    pub fn set_reserve(&mut self, size: usize) {
        self.grid.reserve(size);
    }

    pub fn insert_unit(&mut self, unit_id: Uuid, position: Pos2) {
        let cell = hash_position(position, CELL_SIZE);
        self.grid.entry(cell).or_default().push(unit_id);
    }

    pub fn get_nearby_units(&self, position: Pos2) -> Vec<Uuid> {
        let (cx, cy) = hash_position(position, CELL_SIZE);
        let mut nearby_units = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(units) = self.grid.get(&(cx + dx, cy + dy)) {
                    nearby_units.extend(units);
                }
            }
        }

        nearby_units
    }

    pub fn insert_units(&mut self, units: Vec<(Uuid, Pos2)>) {
        for (unit_id, position) in units {
            self.insert_unit(unit_id, position);
        }
    }

    pub fn remove_unit(&mut self, unit_id: &Uuid, position: Pos2) {
        let cell = hash_position(position, CELL_SIZE);
        if let Some(units) = self.grid.get_mut(&cell) {
            units.retain(|id| id != unit_id);
            if units.is_empty() {
                self.grid.remove(&cell);
            }
        }
    }

    pub(crate) fn update_unit_position_in_grid(
        &mut self,
        unit_id: &Uuid,
        old_position: Pos2,
        new_position: Pos2,
    ) {
        let old_cell = hash_position(old_position, CELL_SIZE);
        let new_cell = hash_position(new_position, CELL_SIZE);

        if old_cell != new_cell {
            self.remove_unit(unit_id, old_position);
            self.insert_unit(*unit_id, new_position);
        }
    }

    pub fn remove_units(&mut self, unit_ids: &Vec<Uuid>) {
        for (_, ids) in self.grid.iter_mut() {
            ids.retain(|id| !unit_ids.contains(id));
        }
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

pub fn hash_position(pos: Pos2, cell_size: f32) -> (i32, i32) {
    ((pos.x / cell_size) as i32, (pos.y / cell_size) as i32)
}
