use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};

#[derive(Clone)]
pub struct CameraState {
    pub camera_pos: Pos2FixedPoint,
    pub target_pos: Pos2FixedPoint,
    pub zoom: i32,
}


impl CameraState {
    pub fn new(camera_pos: Pos2FixedPoint, zoom: i32) -> Self {
        Self {
            camera_pos,
            target_pos: camera_pos,
            zoom: zoom.clamp(256, 4_096),
        }
    }

    pub fn set_zoom(&mut self, zoom: i32) {
        self.zoom = zoom.clamp(256, 4_096);
    }

    pub fn move_camera(&mut self, delta_x: i32, delta_y: i32) {
        self.camera_pos.x += delta_x;
        self.camera_pos.y += delta_y;
        self.target_pos = self.camera_pos;
    }

    pub fn move_to_target(&mut self) {
        self.camera_pos.x = self.target_pos.x;
        self.camera_pos.y = self.target_pos.y;
    }

    pub fn set_target(&mut self, new_target: Pos2FixedPoint) {
        self.target_pos = new_target;
    }

    pub fn update_position(&mut self, delta_time: f64, speed: f32) {
        let factor = (speed * delta_time as f32).clamp(0.0, 1.0);
        if factor > 0.0 {
            self.camera_pos.x += ((self.target_pos.x - self.camera_pos.x) as f32 * factor) as i32;
            self.camera_pos.y += ((self.target_pos.y - self.camera_pos.y) as f32 * factor) as i32;
        }
    }

    pub fn get_zoom_scaled(&self) -> f32 {
        self.zoom as f32 / FIXED_POINT_SCALE as f32
    }
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            camera_pos: Pos2FixedPoint::new(0, 0),
            target_pos: Pos2FixedPoint::new(0, 0),
            zoom: 1_000,
        }
    }
}