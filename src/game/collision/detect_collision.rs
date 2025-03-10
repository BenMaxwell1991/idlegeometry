use crate::game::collision::spatial_hash_grid::SpatialHashGrid;
use crate::game::units::unit::Unit;
use crate::game::units::unit_shape::UnitShape;
use egui::Pos2;
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
