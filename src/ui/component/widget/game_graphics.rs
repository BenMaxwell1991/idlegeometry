use crate::game::data::game_data::GameData;
use crate::game::map::camera_state::CameraState;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::ui::graphics::gl::{draw_map, draw_units};
use crate::ui::graphics::rendering_data::RenderData;
use eframe::egui::{Color32, Sense, Ui, Widget};
use eframe::Frame;
use egui::{Pos2, Rect, Response, Stroke, StrokeKind};
use glow::*;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Instant;

pub struct GameGraphics<'a> {
    game_data: Arc<GameData>,
    frame: &'a mut Frame,
}

impl<'a> GameGraphics<'a> {
    pub fn new(game_data: Arc<GameData>, frame: &'a mut Frame) -> Self {
        Self { game_data, frame }
    }
}

impl<'a> Widget for GameGraphics<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_size = ui.available_size_before_wrap();
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::click());
        let render_data = RenderData::from(Arc::clone(&self.game_data));

        let mut renderer_lock = self.game_data.offscreen_renderer.write().unwrap();
        if let Some(renderer) = renderer_lock.as_mut() {
            renderer.resize(rect.width() as i32, rect.height() as i32);
            let gl = renderer.get_gl();
            let width = rect.width();
            let height = rect.height();

            check_window_size(&self.game_data, rect);

            renderer.bind();
            unsafe {
                gl.viewport(0, 0, width as i32, height as i32);
                gl.clear_color(0.0, 0.0, 0.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            draw_map(&gl, &render_data, &rect, renderer);
            draw_units(&gl, &render_data, &rect, renderer);
            renderer.unbind();

            let texture_id = self.frame.register_native_glow_texture(renderer.get_texture());
            let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
            let tint = Color32::WHITE;
            ui.painter().image(texture_id, rect, uv, tint);
            ui.painter().rect_stroke(rect, 5.0, Stroke::new(3.0, Color32::from_rgb(100, 0, 100)), StrokeKind::Inside);

            return response
        }

        response
    }
}

pub(crate) fn world_to_screen(world_pos: Pos2FixedPoint, camera: &CameraState, rect: &Rect) -> Pos2 {
    Pos2::new(
        ((world_pos.x as i64 - camera.camera_pos.x as i64) as f32 * camera.get_zoom_scaled() / FIXED_POINT_SCALE as f32) + rect.size().x / 2.0,
        ((world_pos.y as i64 - camera.camera_pos.y as i64) as f32 * camera.get_zoom_scaled() / FIXED_POINT_SCALE as f32) + rect.size().y / 2.0,
    )
}

fn check_window_size(game_data: &GameData, rect: Rect) {
    let mut window_size_lock = game_data.graphic_window_size.write().unwrap();

    if let Some(previous_size) = *window_size_lock {
        if rect.size() != previous_size {
            println!("Size Changed: {:?} -> {:?}", previous_size, rect.size());
            *window_size_lock = Some(rect.size());
        }
    } else {
        println!("Set Initial Size: {:?}", rect.size());
        *window_size_lock = Some(rect.size());
    }
}