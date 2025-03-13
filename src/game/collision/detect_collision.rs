use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::{normalize_i64_upscaled, project_onto_i64, Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::units::unit_shape::UnitShape;
use rayon::iter::*;
use rayon::slice::ParallelSliceMut;

pub fn handle_collision(unit_positions_updates: &mut [(u32, Pos2FixedPoint, Pos2FixedPoint)], game_data: &GameData) {
    let spatial_grid = game_data.spatial_hash_grid.read().unwrap();
    let unit_positions = game_data.unit_positions.read().unwrap();
    let units = game_data.units.read().unwrap();
    let chunk_size = ((unit_positions_updates.len() / rayon::current_num_threads()).max(1)).max(1);

    unit_positions_updates
        .par_chunks_mut(chunk_size)
        .for_each(|chunk| {
            for (unit_id, old_position, new_position) in chunk {
                let Some(unit) = units.get(*unit_id as usize).and_then(|u| u.as_ref()) else { continue; };
                let unit_shape = &unit.unit_shape;

                let nearby_unit_ids = spatial_grid.get_nearby_units(*new_position);
                let mut collision_normals = Vec::new();
                let mut nearby_positions = Vec::new();

                for &other_unit_id in &nearby_unit_ids {
                    if *unit_id == other_unit_id {
                        continue;
                    }

                    let Some(other_unit) = units.get(other_unit_id as usize).and_then(|u| u.as_ref()) else { continue; };
                    let other_unit_shape = &other_unit.unit_shape;
                    let other_unit_pos = unit_positions[other_unit_id as usize];

                    if rectangles_collide(*new_position, unit_shape, other_unit_pos, other_unit_shape) {
                        let collision_normal = compute_collision_normal_upscaled(*new_position, unit_shape, other_unit_pos, other_unit_shape);
                        collision_normals.push(collision_normal);
                        nearby_positions.push(other_unit_pos);
                    }
                }

                if collision_normals.is_empty() {
                    continue;
                }

                let resolution_vector = if collision_normals.len() == 1 {
                    project_onto_i64(new_position.sub(*old_position), collision_normals[0])
                } else {
                    let average_normal = sum_vectors(&collision_normals);
                    let normalized = normalize_i64_upscaled(average_normal);
                    project_onto_i64(new_position.sub(*old_position), Pos2FixedPoint { x: normalized.0 as i32, y: normalized.1 as i32 })
                };

                *new_position = Pos2FixedPoint {
                    x: (new_position.x as i64 - resolution_vector.0) as i32,
                    y: (new_position.y as i64 - resolution_vector.1) as i32,
                };

                // Push apart if slightly overlapping
                if !nearby_positions.is_empty() {
                    let separation_vector = compute_separation_vector(*new_position, &nearby_positions);
                    *new_position = new_position.add(separation_vector);
                }
            }
        });
}

fn compute_separation_vector(pos: Pos2FixedPoint, nearby_positions: &[Pos2FixedPoint]) -> Pos2FixedPoint {
    let avg_x = nearby_positions.iter().map(|p| p.x as i64).sum::<i64>() / nearby_positions.len() as i64;
    let avg_y = nearby_positions.iter().map(|p| p.y as i64).sum::<i64>() / nearby_positions.len() as i64;

    let direction_x = pos.x as i64 - avg_x;
    let direction_y = pos.y as i64 - avg_y;

    let (norm_x, norm_y) = normalize_i64_upscaled(Pos2FixedPoint::new(direction_x as i32, direction_y as i32));

    Pos2FixedPoint::new(
        (norm_x * 2) as i32,
        (norm_y * 2) as i32,
    )
}

fn compute_collision_normal_upscaled(pos1: Pos2FixedPoint, shape1: &UnitShape, pos2: Pos2FixedPoint, shape2: &UnitShape) -> Pos2FixedPoint {
    let (min1, max1) = shape1.bounding_box(pos1);
    let (min2, max2) = shape2.bounding_box(pos2);

    let x_overlap = (max1.x - min2.x).min(max2.x - min1.x);
    let y_overlap = (max1.y - min2.y).min(max2.y - min1.y);

    let total_overlap = (x_overlap as i64 + y_overlap as i64).max(1); // Avoid division by zero

    // Compute the normal direction based on overlap ratio
    let mut normal_x = (x_overlap as i64 * FIXED_POINT_SCALE as i64) / total_overlap;
    let mut normal_y = (y_overlap as i64 * FIXED_POINT_SCALE as i64) / total_overlap;

    // Assign sign based on collision direction
    if min1.x > min2.x { normal_x = -normal_x; }
    if min1.y > min2.y { normal_y = -normal_y; }

    Pos2FixedPoint::new(normal_x as i32, normal_y as i32)
}


fn sum_vectors(vectors: &[Pos2FixedPoint]) -> Pos2FixedPoint {
    vectors.iter().fold(Pos2FixedPoint::new(0, 0), |sum, v| sum.add(*v))
}

pub fn rectangles_collide(pos1: Pos2FixedPoint, shape1: &UnitShape, pos2: Pos2FixedPoint, shape2: &UnitShape) -> bool {
    let (min1, max1) = shape1.bounding_box(pos1);
    let (min2, max2) = shape2.bounding_box(pos2);

    let x_overlap = min1.x < max2.x && max1.x > min2.x;
    let y_overlap = min1.y < max2.y && max1.y > min2.y;

    x_overlap && y_overlap
}