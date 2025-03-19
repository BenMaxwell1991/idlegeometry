use std::cmp::max;
use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::{normalize_i64_upscaled, project_onto_i64, Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::units::unit::remove_units;
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use rayon::iter::*;
use rayon::slice::ParallelSliceMut;

use crate::game::resources::loot::collect_loot;
use crate::game::units::loot::Loot;
use crate::helper::lock_helper::acquire_lock;
use std::sync::{Arc, Mutex};
use crate::game::map::game_map::GameMap;
use crate::game::maths::integers::int_sqrt_64;

pub fn handle_collision(unit_positions_updates: &mut [(u32, Pos2FixedPoint, Pos2FixedPoint)], game_data: Arc<GameData>) {
    let collectables_collected = Arc::new(Mutex::new(Vec::new()));

    {
        let units = acquire_lock(&game_data.units, "units");
        let unit_positions = acquire_lock(&game_data.unit_positions, "unit_positions");
        let spatial_grid = acquire_lock(&game_data.spatial_hash_grid, "spatial_grid");
        let game_map = acquire_lock(&game_data.game_map, "game_map");
        let player_id = acquire_lock(&game_data.player_id, "player_id");

        let player_position = player_id
            .and_then(|id| unit_positions.get(id as usize))
            .copied()
            .unwrap_or(Pos2FixedPoint::default());

        let tile_size = game_map.as_ref().map(|m| m.get_tile_size()).unwrap_or(1);
        let chunk_size = ((unit_positions_updates.len() / rayon::current_num_threads()).max(1)).max(1);

        unit_positions_updates
            .par_chunks_mut(chunk_size)
            .for_each(|chunk| {
                for (unit_id, old_position, new_position) in chunk {
                    let Some(unit) = units.get(*unit_id as usize).and_then(|u| u.as_ref()) else { continue; };

                    match unit.unit_type {
                        UnitType::Player => {
                            if let Some(game_map) = game_map.as_ref() {
                                handle_terrain(new_position, old_position, &unit.unit_shape, game_map, tile_size);
                            }
                            continue
                        },
                        UnitType::Collectable => {
                            let collectable_shape = &unit.unit_shape;
                            if rectangles_collide(*new_position, collectable_shape, player_position, &unit.unit_shape) {
                                collectables_collected.lock().unwrap().push(*unit_id);
                            }
                        },

                        _ => {
                            let unit_shape = &unit.unit_shape;
                            let nearby_unit_ids = spatial_grid.get_nearby_units(*new_position);
                            let mut collision_normals = Vec::new();
                            let mut nearby_positions = Vec::new();

                            for &other_unit_id in &nearby_unit_ids {
                                if *unit_id == other_unit_id {
                                    continue;
                                }

                                let Some(other_unit) = units.get(other_unit_id as usize).and_then(|u| u.as_ref()) else { continue; };

                                if other_unit.unit_type == UnitType::Collectable { continue; }

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

                            if let Some(game_map) = game_map.as_ref() {
                                handle_terrain(new_position, old_position, &unit.unit_shape, game_map, tile_size);
                            }
                        }
                    }
                }
            });
    }

    let collected_items = collectables_collected.lock().unwrap();
    let units = acquire_lock(&game_data.units, "");
    let collected_loot: Vec<Loot> = collected_items.iter()
        .filter_map(|&i| units.get(i as usize)?.as_ref()?.loot.clone()) // Unwrap Option<Unit> before accessing loot
        .collect();
    drop(units);

    if !collected_items.is_empty() {
        remove_units(collected_items.clone(), Arc::clone(&game_data));
        collect_loot(collected_loot, Arc::clone(&game_data));
    }
}

pub fn handle_terrain(new_position: &mut Pos2FixedPoint, old_position: &Pos2FixedPoint, unit_shape: &UnitShape, game_map: &GameMap, tile_size: i32) {
    let movement_vector = new_position.sub(*old_position);
    let movement_length_squared = movement_vector.x as i64 * movement_vector.x as i64 + movement_vector.y as i64 * movement_vector.y as i64;
    let step_count = int_sqrt_64((movement_length_squared * 256 / (tile_size * tile_size) as i64).max(1)) as i32;

    let mut adjusted_position = *old_position;
    let step_vector = Pos2FixedPoint::new(
        movement_vector.x / step_count,
        movement_vector.y / step_count
    );

    for _ in 0..step_count {
        adjusted_position = adjusted_position.add(step_vector);
        if check_tile_collision(adjusted_position, unit_shape, game_map, tile_size) {
            let just_x = Pos2FixedPoint::new(new_position.x, old_position.y);
            if !check_tile_collision(just_x, unit_shape, game_map, tile_size) {
                adjusted_position = just_x;
                break;
            }
            let just_y = Pos2FixedPoint::new(old_position.x, new_position.y);
            if !check_tile_collision(just_y, unit_shape, game_map, tile_size) {
                adjusted_position = just_y;
                break;
            }
            adjusted_position = *old_position;
        }
    }

    *new_position = adjusted_position;
}

fn check_tile_collision(pos: Pos2FixedPoint, unit_shape: &UnitShape, game_map: &GameMap, tile_size: i32) -> bool {
    let (unit_min, unit_max) = unit_shape.bounding_box(pos);

    let min_tile_x = unit_min.x / tile_size;
    let min_tile_y = unit_min.y / tile_size;
    let max_tile_x = unit_max.x / tile_size;
    let max_tile_y = unit_max.y / tile_size;

    for tile_x in min_tile_x as usize..=max_tile_x as usize {
        for tile_y in min_tile_y as usize..=max_tile_y as usize {
            let tile = game_map.get_tile(tile_x, tile_y);
            if tile.blocks_collision() {
                let half_tile_size = tile_size / 2;
                let tile_pos = Pos2FixedPoint::new(tile_x as i32 * tile_size + half_tile_size, tile_y as i32 * tile_size + half_tile_size);
                let tile_shape = UnitShape::new(tile_size, tile_size);
                if rectangles_collide(pos, unit_shape, tile_pos, &tile_shape) {
                    return true;
                }
            }
        }
    }

    false
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

    let total_overlap = (x_overlap as i64 + y_overlap as i64).max(1);

    let mut normal_x = (x_overlap as i64 * FIXED_POINT_SCALE as i64) / total_overlap;
    let mut normal_y = (y_overlap as i64 * FIXED_POINT_SCALE as i64) / total_overlap;

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