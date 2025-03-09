use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CAMERA_STATE, GAME_MAP, SPRITE_SHEETS, UNITS};
use crate::game::map::camera_state::CameraState;
use crate::game::map::tile_type::TileType;
use eframe::egui::{Color32, Id, Sense, Ui, Vec2, Widget};
use egui::{Pos2, Rect, Response, Stroke, StrokeKind};
use std::hash::Hash;
use std::sync::Arc;

pub struct GameGraphics {
    game_data: Arc<GameData>,
    id: Id,
}

impl GameGraphics {
    pub fn new(game_data: Arc<GameData>, id_salt: impl Hash) -> Self {
        Self {
            game_data,
            id: Id::new(id_salt),
        }
    }

    fn draw_map(&self, painter: &egui::Painter, rect: &Rect, camera_state: &CameraState) {
        if let Some(game_map) = self.game_data.get_field(GAME_MAP) {
            let tile_size = game_map.tile_size * camera_state.zoom;

            for (&(x, y), tile) in &game_map.tiles {
                let world_pos = Pos2::new(x as f32 * game_map.tile_size, y as f32 * game_map.tile_size);
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

    fn draw_units(&self, painter: &egui::Painter, rect: &Rect, camera_state: &CameraState) {
        if let Some(units) = self.game_data.get_field(UNITS) {
            for unit in units {
                let unit_screen_pos = world_to_screen(unit.position, camera_state, rect);
                let unit_size = Vec2::new(20.0, 20.0) * camera_state.zoom;
                let unit_rect = Rect::from_center_size(unit_screen_pos, unit_size);

                if let Some(sprite_sheets) = self.game_data.get_field(SPRITE_SHEETS) {
                    if let Some(sprite_sheet) = sprite_sheets.get(&unit.animation.sprite_key) {
                        let frame_index = (unit.animation.animation_frame * sprite_sheet.get_frame_count() as f32).trunc() as usize;
                        let frame = sprite_sheet.get_frame(frame_index);
                        painter.image(frame.id(), unit_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
                    }
                }
            }
        }
    }
}

impl Widget for GameGraphics {
    fn ui(self, ui: &mut Ui) -> Response {
        let camera_state = self.game_data.get_field(CAMERA_STATE).unwrap_or(CameraState::default());
        let available_size = ui.available_size_before_wrap();
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::click());
        let mut painter = ui.painter().clone();
        painter.set_clip_rect(rect);

        self.draw_map(&painter, &rect, &camera_state);
        self.draw_units(&painter, &rect, &camera_state);

        painter.rect_stroke(rect, 5.0, Stroke::new(3.0, Color32::from_rgb(100, 0, 100)), StrokeKind::Inside);

        response
    }
}

fn world_to_screen(world_pos: Pos2, camera: &CameraState, rect: &Rect) -> Pos2 {
    Pos2::new(
        (world_pos.x - camera.camera_pos.x) * camera.zoom + rect.center().x,
        (world_pos.y - camera.camera_pos.y) * camera.zoom + rect.center().y,
    )
}