use eframe::egui::Pos2;

#[derive(Clone)]
pub struct CameraState {
    pub camera_pos: Pos2,
    pub zoom: f32,
}

impl CameraState {
    pub fn new(camera_pos: Pos2, zoom: f32) -> Self {
        Self {
            camera_pos,
            zoom,
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.2, 5.0); //
    }

    pub fn move_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.camera_pos.x += delta_x;
        self.camera_pos.y += delta_y;
    }
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            camera_pos: Pos2::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}