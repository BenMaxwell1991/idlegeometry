use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CAMERA_STATE, GAME_MAP, SPRITE_SHEETS};
use crate::game::map::camera_state::CameraState;
use crate::game::map::tile_type::TileType;
use crate::game::units::unit_type::UnitType;
use eframe::egui::{Color32, Id, Sense, Ui, Vec2, Widget};
use egui::{Painter, Pos2, Rect, Response, Stroke, StrokeKind};
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

    fn draw_units(&self, painter: &Painter, rect: &Rect, camera_state: &CameraState) {
        let sprite_sheets = self.game_data.get_field(SPRITE_SHEETS).unwrap();
        let units = self.game_data.units.read().unwrap();
        let unit_screen_positions: Vec<_> = units.iter()
            .map(|unit| world_to_screen(unit.position, camera_state, rect))
            .collect();

        let mut images_to_draw = Vec::new();
        let mut rects_to_draw = Vec::new();
        let mut player_to_draw = Vec::new();

        for (unit, unit_screen_position) in units.iter().zip(unit_screen_positions) {
            if !rect.contains(unit_screen_position) { continue; }

            let unit_size = Vec2::new(20.0, 20.0) * camera_state.zoom;
            let unit_rect = Rect::from_center_size(unit_screen_position, unit_size);

            if (unit_size.x < 5.0 || unit_size.y < 5.0) && unit.unit_type != UnitType::Player {
                rects_to_draw.push(unit_rect);
                continue;
            }

            if let Some(sprite_sheet) = sprite_sheets.get(&unit.animation.sprite_key) {
                let frame_index = (unit.animation.animation_frame * sprite_sheet.get_frame_count() as f32).trunc() as usize;
                let frame = sprite_sheet.get_frame(frame_index);
                match unit.unit_type {
                    UnitType::Player => { player_to_draw.push((frame.id(), unit_rect)); }
                    UnitType::Enemy => { images_to_draw.push((frame.id(), unit_rect)); }
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

impl Widget for GameGraphics {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let camera_state = self.game_data.get_field(CAMERA_STATE).unwrap_or(CameraState::default());
        let available_size = ui.available_size_before_wrap();
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::click());

        let mut painter = ui.painter().clone();
        painter.set_clip_rect(rect);

        let game_data_one = Arc::clone(&self.game_data);
        check_window_size(game_data_one, rect);

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