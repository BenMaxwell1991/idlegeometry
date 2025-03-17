use crate::game::maths::pos_2::Pos2FixedPoint;
use rayon::iter::*;
use rayon::prelude::ParallelSlice;
use rustc_hash::*;
use serde::{Deserialize, Serialize};

const CELL_SIZE_BITS: i32 = 17; // 2^14 = 131,072

#[derive(Clone, Serialize, Deserialize)]
pub struct SpatialHashGrid {
    pub grid: FxHashMap<(i32, i32), Vec<u32>>,
}

impl SpatialHashGrid {
    pub fn new() -> Self {
        Self { grid: FxHashMap::default() }
    }

    pub fn insert_unit(&mut self, unit_id: u32, position: Pos2FixedPoint) {
        let cell = hash_position(position);
        self.grid.entry(cell).or_default().push(unit_id);
    }

    pub fn remove_unit(&mut self, unit_id: &u32, position: Pos2FixedPoint) {
        let cell = hash_position(position);
        if let Some(units) = self.grid.get_mut(&cell) {
            units.retain(|id| id != unit_id);
            if units.is_empty() {
                self.grid.remove(&cell);
            }
        }
    }

    pub fn get_nearby_units(&self, position: Pos2FixedPoint) -> Vec<u32> {
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

    pub fn update_units_position_in_grid(&mut self, updates: &[(u32, Pos2FixedPoint, Pos2FixedPoint)]) {
        let chunk_size = (updates.len() / rayon::current_num_threads().max(1)).max(1);
        let thread_local_maps: Vec<_> = updates
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut local_grid: FxHashMap<(i32, i32), Vec<u32>> = FxHashMap::default();
                for &(unit_id, _, new_position) in chunk {
                    local_grid.entry(hash_position(new_position)).or_insert_with(Vec::new).push(unit_id);
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

    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

pub fn hash_position(pos: Pos2FixedPoint) -> (i32, i32) {
    ((pos.x >> CELL_SIZE_BITS), (pos.y >> CELL_SIZE_BITS))
}
