use crate::game::data::game_data::GameData;
use crate::game::maths::pos_2::{normalize_i64_upscaled, project_onto_i64, Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::objects::game_object::{add_units, remove_units, GameObject};
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use rayon::iter::*;
use rayon::slice::ParallelSliceMut;

use crate::game::data::damage_numbers::DamageNumber;
use crate::game::map::game_map::GameMap;
use crate::game::maths::integers::int_sqrt_64;
use crate::game::objects::attacks::attack_landed::AttackLanded;
use crate::game::objects::attacks::attack_stats::AttackStats;
use crate::game::objects::loot::Loot;
use crate::game::resources::loot::collect_loot;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use egui::Color32;
use rustc_hash::FxHashSet;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use smallvec::SmallVec;

pub fn handle_collision(unit_positions_updates: &mut [(u32, Pos2FixedPoint, Pos2FixedPoint)], game_data: Arc<GameData>, delta_time: f64) {
    let collectables_collected = Arc::new(Mutex::new(Vec::new()));
    let mut units_to_remove = FxHashSet::default();

    {
        let mut units = acquire_lock_mut(&game_data.units, "objects");
        let unit_positions = acquire_lock_mut(&game_data.unit_positions, "unit_positions");
        let spatial_grid = acquire_lock_mut(&game_data.spatial_hash_grid, "spatial_grid");
        let game_map = acquire_lock_mut(&game_data.game_map, "game_map");

        let player_id = acquire_lock_mut(&game_data.player_id, "player_id");
        let player_position = player_id
            .and_then(|id| unit_positions.get(id as usize))
            .copied()
            .unwrap_or(Pos2FixedPoint::default());

        let tile_size = game_map.as_ref().map(|m| m.get_tile_size()).unwrap_or(1);
        let chunk_size = ((unit_positions_updates.len() / rayon::current_num_threads()).max(1)).max(1);

        let mut attack_hits_to_process = Arc::new(Mutex::new(Vec::default()));

        unit_positions_updates
            .par_chunks_mut(chunk_size)
            .for_each(|chunk| {
                let mut nearby_unit_ids = SmallVec::<[u32; 64]>::new();

                for (unit_id, old_position, new_position) in chunk {
                    let Some(unit) = units.get(*unit_id as usize).and_then(|u| u.as_ref()) else { continue; };

                    match unit.object_type {
                        ObjectType::Player => {
                            if let Some(game_map) = game_map.as_ref() {
                                handle_terrain(new_position, old_position, &unit.object_shape, game_map, tile_size);
                            }
                            continue
                        },
                        ObjectType::Collectable => {
                            let collectable_shape = &unit.object_shape;
                            if rectangles_collide(*new_position, collectable_shape, player_position, &unit.object_shape) {
                                collectables_collected.lock().unwrap().push(*unit_id);
                            }
                        },
                        ObjectType::Attack => {
                            if let Some(attack_stats) = &unit.attack_stats {
                                if is_in_damage_window(&attack_stats, delta_time) {
                                    let attack = unit;
                                    let attack_id = unit.id;
                                    let attack_pos = unit_positions[attack_id as usize];
                                    let attack_shape = &unit.object_shape;

                                    spatial_grid.get_nearby_units_into(*new_position, &mut nearby_unit_ids);
                                    for &nearby_unit_id in &nearby_unit_ids {
                                        let Some(nearby_unit) = units.get(nearby_unit_id as usize).and_then(|u| u.as_ref()) else { continue; };

                                        // Attacks don't hit collectables or other attacks
                                        if nearby_unit.object_type == ObjectType::Collectable || nearby_unit.object_type == ObjectType::Attack {
                                            continue
                                        }

                                        // Attacks don't hit their parents
                                        if Some(nearby_unit_id) == attack.parent_unit_id {
                                            continue;
                                        }

                                        // Attacks only hit objects once
                                        if attack_stats.units_hit.contains(&nearby_unit_id) {
                                            continue;
                                        }

                                        let nearby_unit_pos = unit_positions[nearby_unit_id as usize];
                                        if rectangles_collide(attack_pos, &attack_shape, nearby_unit_pos, &nearby_unit.object_shape) {
                                            let attack_to_process = AttackLanded {
                                                attack_id: attack.id,
                                                target_id: nearby_unit_id,
                                                damage: attack_stats.damage,
                                            };
                                            attack_hits_to_process.lock().unwrap().push(attack_to_process);
                                        }
                                    }
                                }
                            }
                        },
                        _ => {
                            let unit_shape = &unit.object_shape;

                            // let now = Instant::now();
                            spatial_grid.get_nearby_units_into(*new_position, &mut nearby_unit_ids);
                            // println!("elapsed 1: {}", now.elapsed().as_nanos());

                            let mut collision_normals = Vec::new();
                            let mut nearby_positions = Vec::new();

                            for &other_unit_id in &nearby_unit_ids {
                                if *unit_id == other_unit_id {
                                    continue;
                                }

                                let Some(other_unit) = units.get(other_unit_id as usize).and_then(|u| u.as_ref()) else { continue; };

                                if other_unit.object_type == ObjectType::Collectable { continue; }

                                let other_unit_shape = &other_unit.object_shape;
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
                                handle_terrain(new_position, old_position, &unit.object_shape, game_map, tile_size);
                            }
                        }
                    }
                }
            });

        let mut damage_numbers = acquire_lock_mut(&game_data.damage_numbers, "damage_numbers");
        for attack_to_process in attack_hits_to_process.lock().unwrap().iter() {
            let attack_id = attack_to_process.attack_id as usize;
            let target_id = attack_to_process.target_id as usize;

            if units_to_remove.contains(&attack_to_process.target_id) {
                continue;
            }

            let (attack, target) = if attack_id < target_id {
                let (low, high) = units.split_at_mut(target_id);
                (low.get_mut(attack_id), high.get_mut(0))
            } else {
                let (low, high) = units.split_at_mut(attack_id);
                (high.get_mut(0), low.get_mut(target_id))
            };

            if let (Some(Some(attack)), Some(Some(target))) = (attack, target) {
                if let Some(attack_stats) = attack.attack_stats.as_mut() {
                    if attack_stats.hit_count < attack_stats.max_targets {
                        let is_dead = target.apply_damage(attack_to_process.damage);
                        attack_stats.units_hit.push(attack_to_process.target_id);
                        attack_stats.hit_count += 1;

                        if is_dead {
                            units_to_remove.insert(attack_to_process.target_id);
                        } else {
                            if let Some(target_pos) = unit_positions.get(target_id) {
                                let damage_number = DamageNumber {
                                    position: target_pos.clone(),
                                    value: attack_stats.damage,
                                    spawn_time: Instant::now(),
                                    colour: Color32::RED,
                                };
                                damage_numbers.push(damage_number);
                            }
                        }
                    }
                }
            }
        }
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

    let mut units_to_remove_vec: Vec<u32> = units_to_remove.into_iter().collect();

    if !units_to_remove_vec.is_empty() {
        let collectables = remove_units(units_to_remove_vec, Arc::clone(&game_data));
        let (collectable_units, collectable_positions): (Vec<GameObject>, Vec<Pos2FixedPoint>) = collectables.into_iter().unzip();
        add_units(collectable_units, collectable_positions, &game_data);
    }
}

pub fn handle_terrain(new_position: &mut Pos2FixedPoint, old_position: &Pos2FixedPoint, unit_shape: &ObjectShape, game_map: &GameMap, tile_size: i32) {
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

fn check_tile_collision(pos: Pos2FixedPoint, unit_shape: &ObjectShape, game_map: &GameMap, tile_size: i32) -> bool {
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
                let tile_shape = ObjectShape::new(tile_size, tile_size);
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

fn compute_collision_normal_upscaled(pos1: Pos2FixedPoint, shape1: &ObjectShape, pos2: Pos2FixedPoint, shape2: &ObjectShape) -> Pos2FixedPoint {
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

pub fn rectangles_collide(pos1: Pos2FixedPoint, shape1: &ObjectShape, pos2: Pos2FixedPoint, shape2: &ObjectShape) -> bool {
    let (min1, max1) = shape1.bounding_box(pos1);
    let (min2, max2) = shape2.bounding_box(pos2);

    let x_overlap = min1.x < max2.x && max1.x > min2.x;
    let y_overlap = min1.y < max2.y && max1.y > min2.y;

    x_overlap && y_overlap
}

fn is_in_damage_window(attack: &AttackStats, delta_time: f64) -> bool {
    let current_time = attack.elapsed;
    let next_time = attack.elapsed + delta_time as f32;

    let damage_start = attack.damage_point;
    let damage_end = attack.damage_point + attack.damage_duration;

    let in_window = current_time >= damage_start && current_time <= damage_end;
    let will_miss_next_window = current_time < damage_start && next_time > damage_end;

    in_window || will_miss_next_window
}