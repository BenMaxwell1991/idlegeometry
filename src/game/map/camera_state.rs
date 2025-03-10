use eframe::egui::Pos2;

#[derive(Clone)]
pub struct CameraState {
    pub camera_pos: Pos2,
    pub target_pos: Pos2,
    pub zoom: f32,
}


impl CameraState {
    pub fn new(camera_pos: Pos2, zoom: f32) -> Self {
        Self {
            camera_pos,
            target_pos: camera_pos,
            zoom,
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.2, 5.0);
    }

    pub fn move_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.camera_pos.x += delta_x;
        self.camera_pos.y += delta_y;
        self.target_pos = self.camera_pos;
    }

    pub fn set_target(&mut self, new_target: Pos2) {
        self.target_pos = new_target;
    }

    pub fn update_position(&mut self, delta_time: f64, speed: f32) {
        let factor = (speed * delta_time as f32).clamp(0.0, 1.0); // Clamp to prevent overshooting
        self.camera_pos.x += (self.target_pos.x - self.camera_pos.x) * factor;
        self.camera_pos.y += (self.target_pos.y - self.camera_pos.y) * factor;
    }
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            camera_pos: Pos2::new(0.0, 0.0),
            target_pos: Pos2::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}