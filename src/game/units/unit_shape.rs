use egui::Pos2;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UnitShape {
    pub width: f32,
    pub height: f32,
}

impl UnitShape {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn bounding_box(&self, position: Pos2) -> (Pos2, Pos2) {
        (
            Pos2::new(position.x - self.width / 2.0, position.y - self.height / 2.0),
            Pos2::new(position.x + self.width / 2.0, position.y + self.height / 2.0),
        )
    }
}
