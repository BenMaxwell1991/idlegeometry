use crate::game::collision::spatial_hash_grid::SpatialHashGrid;
use crate::game::units::unit::Unit;
use crate::game::units::unit_shape::UnitShape;
use egui::{Pos2, Vec2};
use uuid::Uuid;

/// Checks if a unit would collide with any nearby unit in `SpatialHashGrid`
pub fn check_unit_collision(
    unit_id: &Uuid,
    new_position: Pos2,
    unit_shape: &UnitShape,
    spatial_grid: &SpatialHashGrid,
    game_units: &[Unit],
) -> bool {
    let nearby_units = spatial_grid.get_nearby_units(new_position);

    for other_id in nearby_units {
        if unit_id != &other_id {
            if let Some(other_unit) = game_units.iter().find(|u| u.id == other_id) {
                if rectangles_collide(new_position, unit_shape, other_unit.position, &other_unit.unit_shape) {
                    return true;
                }
            }
        }
    }
    false
}

/// Checks if two rectangles collide (Axis-Aligned Bounding Box)
pub fn rectangles_collide(pos1: Pos2, shape1: &UnitShape, pos2: Pos2, shape2: &UnitShape) -> bool {
    let (min1, max1) = shape1.bounding_box(pos1);
    let (min2, max2) = shape2.bounding_box(pos2);

    let x_overlap = min1.x < max2.x && max1.x > min2.x;
    let y_overlap = min1.y < max2.y && max1.y > min2.y;

    x_overlap && y_overlap
}

pub fn swept_collision_check(
    unit_id: &Uuid,
    old_position: Pos2,
    intended_position: Pos2,
    unit_shape: &UnitShape,
    spatial_grid: &SpatialHashGrid,
    game_units: &[Unit],
) -> Option<Pos2> {
    let nearby_units = spatial_grid.get_nearby_units(intended_position);

    for other_id in nearby_units {
        if unit_id != &other_id {
            if let Some(other_unit) = game_units.iter().find(|u| u.id == other_id) {
                let collision_point = segment_vs_rect_collision(
                    old_position,
                    intended_position,
                    unit_shape,
                    other_unit.position,
                    &other_unit.unit_shape
                );

                if let Some(adjusted_position) = collision_point {
                    let slide_position = slide_along_surface(
                        old_position,
                        intended_position,
                        adjusted_position,
                        unit_shape,
                        &other_unit.unit_shape
                    );

                    return Some(slide_position);
                }
            }
        }
    }
    None
}

pub fn segment_vs_rect_collision(
    start: Pos2,
    end: Pos2,
    shape: &UnitShape,
    rect_pos: Pos2,
    rect_shape: &UnitShape,
) -> Option<Pos2> {
    let (min1, max1) = shape.bounding_box(start);
    let (min2, max2) = rect_shape.bounding_box(rect_pos);

    // Cast a ray from start to end, check if it intersects the bounding box of another unit
    if ray_intersects_aabb(start, end, min2, max2) {
        return Some(start); // Stop movement at the last safe position
    }
    None
}

pub fn ray_intersects_aabb(ray_start: Pos2, ray_end: Pos2, min: Pos2, max: Pos2) -> bool {
    let direction = ray_end - ray_start;
    let inv_direction = Vec2::new(
        if direction.x != 0.0 { 1.0 / direction.x } else { f32::INFINITY },
        if direction.y != 0.0 { 1.0 / direction.y } else { f32::INFINITY },
    );

    let t1 = (min.x - ray_start.x) * inv_direction.x;
    let t2 = (max.x - ray_start.x) * inv_direction.x;
    let t3 = (min.y - ray_start.y) * inv_direction.y;
    let t4 = (max.y - ray_start.y) * inv_direction.y;

    let t_min = t1.min(t2).max(t3.min(t4)); // Entry time
    let t_max = t1.max(t2).min(t3.max(t4)); // Exit time

    t_max >= 0.0 && t_min <= t_max && t_min <= 1.0
}

pub fn slide_along_surface(
    old_position: Pos2,
    intended_position: Pos2,
    collision_position: Pos2,
    unit_shape: &UnitShape,
    obstacle_shape: &UnitShape
) -> Pos2 {
    let movement_vector = intended_position - old_position; // Direction of movement
    let obstacle_normal = compute_collision_normal(old_position, obstacle_shape); // Normal of the obstacle

    // Project movement onto the surface normal
    let slide_vector = movement_vector - (movement_vector.dot(obstacle_normal) * obstacle_normal);

    // New position is adjusted based on the slide vector
    let new_slide_position = collision_position + slide_vector;

    new_slide_position
}

pub fn compute_collision_normal(unit_position: Pos2, obstacle_shape: &UnitShape) -> Vec2 {
    let (min, max) = obstacle_shape.bounding_box(unit_position);

    if unit_position.x <= min.x {
        return Vec2::new(-1.0, 0.0); // Left wall
    }
    if unit_position.x >= max.x {
        return Vec2::new(1.0, 0.0); // Right wall
    }
    if unit_position.y <= min.y {
        return Vec2::new(0.0, -1.0); // Bottom wall
    }
    if unit_position.y >= max.y {
        return Vec2::new(0.0, 1.0); // Top wall
    }

    Vec2::new(0.0, 0.0) // No normal (fallback)
}
