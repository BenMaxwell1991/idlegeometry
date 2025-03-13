use crate::game::maths::pos_2::Pos2FixedPoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UnitShape {
    pub width: i32,
    pub height: i32,
}

impl UnitShape {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn bounding_box(&self, position: Pos2FixedPoint) -> (Pos2FixedPoint, Pos2FixedPoint) {
        let half_width = self.width >> 1;
        let half_height = self.height >> 1;
        (
            Pos2FixedPoint::new(position.x - half_width, position.y - half_height),
            Pos2FixedPoint::new(position.x + half_width, position.y + half_height),
        )
    }
}
