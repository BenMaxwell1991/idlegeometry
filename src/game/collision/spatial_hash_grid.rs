use egui::Pos2;
use rayon::iter::*;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use rayon::prelude::ParallelSlice;

const CELL_SIZE: f32 = 20.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct SpatialHashGrid {
    pub grid: FxHashMap<(i32, i32), Vec<u32>>,
}

impl SpatialHashGrid {
    pub fn new() -> Self {
        Self { grid: FxHashMap::default() }
    }

    pub fn set_reserve(&mut self, size: usize) {
        self.grid.reserve(size);
    }

    pub fn insert_unit(&mut self, unit_id: u32, position: Pos2) {
        let cell = hash_position(position);
        self.grid.entry(cell).or_default().push(unit_id);
    }

    pub fn remove_unit(&mut self, unit_id: &u32, position: Pos2) {
        let cell = hash_position(position);
        if let Some(units) = self.grid.get_mut(&cell) {
            units.retain(|id| id != unit_id);
            if units.is_empty() {
                self.grid.remove(&cell);
            }
        }
    }

    pub fn get_nearby_units(&self, position: Pos2) -> Vec<u32> {
        let (cx, cy) = hash_position(position);
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

    pub fn insert_units(&mut self, units: Vec<(u32, Pos2)>) {
        for (unit_id, position) in units {
            self.insert_unit(unit_id, position);
        }
    }


    pub fn update_units_position_in_grid(&mut self, updates: &[(u32, Pos2, Pos2)]) {
        let thread_local_maps: Vec<_> = updates
            .par_chunks(updates.len() / rayon::current_num_threads().max(1))
            .map(|chunk| {
                let mut local_grid: FxHashMap<(i32, i32), Vec<u32>> = FxHashMap::default();
                for &(unit_id, _, new_position) in chunk {
                    local_grid.entry(hash_position(new_position)).or_default().push(unit_id);
                }
                local_grid
            })
            .collect();

        let mut new_grid: FxHashMap<(i32, i32), Vec<u32>> = FxHashMap::default();
        for local_map in thread_local_maps {
            for (cell, unit_list) in local_map {
                new_grid.entry(cell).or_default().extend(unit_list);
            }
        }

        self.grid = new_grid;
    }

    pub(crate) fn update_unit_position_in_grid(
        &mut self,
        unit_id: &u32,
        old_position: &Pos2,
        new_position: &Pos2,
    ) {
        let old_cell = hash_position(*old_position);
        let new_cell = hash_position(*new_position);

        if old_cell != new_cell {
            self.remove_unit(unit_id, *old_position);
            self.insert_unit(*unit_id, *new_position);
        }
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

pub fn hash_position(pos: Pos2) -> (i32, i32) {
    ((pos.x / CELL_SIZE) as i32, (pos.y / CELL_SIZE) as i32)
}
