use std::time::Instant;
use crate::game::units::unit_shape::UnitShape;
use egui::Pos2;
use rayon::iter::*;
use rayon::slice::ParallelSliceMut;
use crate::game::data::game_data::GameData;

pub fn handle_collision(unit_positions_updates: &mut [(u32, Pos2, Pos2)], game_data: &GameData) {
    let spatial_grid = game_data.spatial_hash_grid.read().unwrap();
    let unit_positions = game_data.unit_positions.read().unwrap();
    let units = game_data.units.read().unwrap();

    let chunk_size = (unit_positions_updates.len() / rayon::current_num_threads()).max(1);

    let now = Instant::now();
    unit_positions_updates
        .par_chunks_mut(chunk_size)
        .for_each(|chunk| {
            for (unit_id, old_position, new_position) in chunk {
                let Some(unit) = units.get(*unit_id as usize).and_then(|u| u.as_ref()) else { continue; };
                let unit_shape = &unit.unit_shape;

                let nearby_unit_ids = spatial_grid.get_nearby_units(*new_position);

                for &other_unit_id in &nearby_unit_ids {
                    if *unit_id == other_unit_id {
                        continue;
                    }

                    let Some(other_unit) = units.get(other_unit_id as usize).and_then(|u| u.as_ref()) else { continue; };
                    let other_unit_shape = &other_unit.unit_shape;
                    let other_unit_pos = unit_positions[other_unit_id as usize];

                    if rectangles_collide(*new_position, unit_shape, other_unit_pos, other_unit_shape) {
                        *new_position = *old_position; // Revert movement on collision
                        break; // Exit early once collision is detected
                    }
                }
            }
        });
    println!("Checked all collisions in: {}", now.elapsed().as_micros());
}

pub fn rectangles_collide(pos1: Pos2, shape1: &UnitShape, pos2: Pos2, shape2: &UnitShape) -> bool {
    let (min1, max1) = shape1.bounding_box(pos1);
    let (min2, max2) = shape2.bounding_box(pos2);

    let x_overlap = min1.x < max2.x && max1.x > min2.x;
    let y_overlap = min1.y < max2.y && max1.y > min2.y;

    x_overlap && y_overlap
}