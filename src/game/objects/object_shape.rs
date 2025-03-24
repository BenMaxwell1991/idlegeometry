use crate::game::maths::pos_2::Pos2FixedPoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ObjectShape {
    pub width: i32,
    pub height: i32,
}

impl ObjectShape {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn bounding_box(&self, position: Pos2FixedPoint) -> (Pos2FixedPoint, Pos2FixedPoint) {
        let half_width = self.width >> 1;
        let half_height = self.height >> 1;

        debug_assert!(
            position.x >= i32::MIN + half_width && position.y >= i32::MIN + half_height,
            "Underflow in bounding_box! pos=({},{}) shape=({}, {})",
            position.x, position.y, self.width, self.height
        );

        (
            Pos2FixedPoint::new(position.x - half_width, position.y - half_height),
            Pos2FixedPoint::new(position.x + half_width, position.y + half_height),
        )
    }
}
