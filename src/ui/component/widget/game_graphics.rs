use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{GAME_MAP, SPRITE_SHEETS};
use crate::game::map::camera_state::CameraState;
use crate::game::map::tile_type::TileType;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::units::unit_type::UnitType;
use eframe::egui::{Color32, Sense, Ui, Vec2, Widget};
use eframe::Frame;
use egui::{Painter, Pos2, Rect, Response, Stroke, StrokeKind};
use std::hash::Hash;
use std::sync::Arc;
use glow::*;
use crate::ui::graphics::gl::{create_shader_program, draw_map};

pub struct GameGraphics<'a> {
    game_data: Arc<GameData>,
    frame: &'a mut Frame,
}

impl<'a> GameGraphics<'a> {
    pub fn new(game_data: Arc<GameData>, frame: &'a mut Frame) -> Self {
        Self { game_data, frame }
    }

    fn draw_map_old(&self, painter: &Painter, rect: &Rect, camera_state: &CameraState) {
        if let Some(game_map) = self.game_data.get_field(GAME_MAP) {
            let tile_size =  camera_state.get_zoom_scaled() * game_map.tile_size as f32 / FIXED_POINT_SCALE as f32;

            for (&(x, y), tile) in &game_map.tiles {
                let world_pos = Pos2FixedPoint::new(x as i32 * game_map.tile_size, y as i32 * game_map.tile_size);
                let screen_pos = world_to_screen(world_pos, camera_state, rect);

                let tile_rect = Rect::from_min_size(screen_pos, Vec2::new(tile_size, tile_size));

                let color = match tile.tile_type {
                    TileType::Wall => Color32::DARK_GRAY,
                    TileType::SpawnPoint => Color32::from_rgb(0, 0, 100),
                    TileType::Empty => Color32::TRANSPARENT,
                };

                painter.rect_filled(tile_rect, 2.0, color);
            }
        }
    }

    fn draw_units(&self, painter: &Painter, rect: &Rect, camera_state: &CameraState) {
        let sprite_sheets = self.game_data.get_field(SPRITE_SHEETS);
        let units_lock = self.game_data.units.read().unwrap();
        let unit_positions_lock = self.game_data.unit_positions.read().unwrap();

        let mut images_to_draw = Vec::new();
        let mut rects_to_draw = Vec::new();
        let mut player_to_draw = Vec::new();

        for unit_option in units_lock.iter() {
            if let Some(unit) = unit_option {
                let unit_screen_position = world_to_screen(unit_positions_lock[unit.id as usize], camera_state, rect);

                if !rect.contains(unit_screen_position) {
                    continue;
                }

                let unit_size = Vec2::new(20.0, 20.0) * camera_state.get_zoom_scaled();
                let unit_rect = Rect::from_center_size(unit_screen_position, unit_size);

                if (unit_size.x < 5.0 || unit_size.y < 5.0) && unit.unit_type != UnitType::Player {
                    rects_to_draw.push(unit_rect);
                    continue;
                }

                if let Some(sprite_sheets) = sprite_sheets.as_ref() {
                    if let Some(sprite_sheet) = sprite_sheets.get(&unit.animation.sprite_key) {
                        let frame_index = (unit.animation.animation_frame * sprite_sheet.get_frame_count() as f32).trunc() as usize;
                        let frame = sprite_sheet.get_frame(frame_index);
                        match unit.unit_type {
                            UnitType::Player => player_to_draw.push((frame.id(), unit_rect)),
                            UnitType::Enemy => images_to_draw.push((frame.id(), unit_rect)),
                        }
                    }
                }
            }
        }

        for unit_rect in rects_to_draw {
            painter.rect_filled(unit_rect, 1.0, Color32::RED);
        }

        for (image_id, unit_rect) in images_to_draw {
            painter.image(image_id, unit_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        }

        for (image_id, unit_rect) in player_to_draw {
            painter.image(image_id, unit_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        }
    }
}

impl<'a> Widget for GameGraphics<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_size = ui.available_size_before_wrap();
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::click());

        let mut renderer_lock = self.game_data.offscreen_renderer.write().unwrap();
        if let Some(renderer) = renderer_lock.as_ref() {
            let gl = renderer.get_gl();
            let width = rect.width();
            let height = rect.height();

            let game_data_one = Arc::clone(&self.game_data);
            check_window_size(game_data_one, rect);
            let camera_state_cloned = self.game_data.camera_state.read().unwrap().clone();

            renderer.bind();
            unsafe {
                gl.viewport(0, 0, width as i32, height as i32);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                gl.clear_color(0.0, 0.0, 0.0, 1.0);
                // draw_red_rectangle(&gl, width , height);
                draw_map(gl, &self.game_data, &rect, &camera_state_cloned);
            }
            renderer.unbind();

            let texture_id = self.frame.register_native_glow_texture(renderer.get_texture());
            let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
            let tint = Color32::WHITE;
            ui.painter().image(texture_id, rect, uv, tint);
            ui.painter().rect_stroke(rect, 5.0, Stroke::new(3.0, Color32::from_rgb(100, 0, 100)), StrokeKind::Inside);

            return response
        }

        // let mut painter = ui.painter().clone();
        // painter.set_clip_rect(rect);
        //
        // let game_data_one = Arc::clone(&self.game_data);
        // check_window_size(game_data_one, rect);
        // let camera_state_cloned = self.game_data.camera_state.read().unwrap().clone();
        //
        // self.draw_map(&painter, &rect, &camera_state_cloned);
        // self.draw_units(&painter, &rect, &camera_state_cloned);
        //
        // painter.rect_stroke(rect, 5.0, Stroke::new(3.0, Color32::from_rgb(100, 0, 100)), StrokeKind::Inside);

        response
    }
}

pub(crate) fn world_to_screen(world_pos: Pos2FixedPoint, camera: &CameraState, rect: &Rect) -> Pos2 {
    Pos2::new(
        ((world_pos.x as i64 - camera.camera_pos.x as i64) as f32 * camera.get_zoom_scaled() / FIXED_POINT_SCALE as f32) + rect.center().x,
        ((world_pos.y as i64 - camera.camera_pos.y as i64) as f32 * camera.get_zoom_scaled() / FIXED_POINT_SCALE as f32) + rect.center().y,
    )
}

fn check_window_size(game_data: Arc<GameData>, rect: Rect) {
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