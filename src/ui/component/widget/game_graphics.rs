use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CAMERA_STATE, GAME_MAP, PLAYER_POSITION};
use crate::game::map::camera_state::CameraState;
use crate::game::map::tile_type::TileType;
use eframe::egui::{Color32, Id, Sense, Ui, Vec2, Widget};
use egui::{Pos2, Rect, Response};
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
}

impl Widget for GameGraphics {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::click());
        let painter = ui.painter();

        let game_map = self.game_data.get_field(GAME_MAP).unwrap();
        let camera_state = self.game_data.get_field(CAMERA_STATE).unwrap_or(CameraState::default());
        let tile_size = game_map.tile_size * camera_state.zoom; // Apply zoom

        for (&(x, y), tile) in &game_map.tiles {
            let world_pos = Pos2::new(x as f32 * game_map.tile_size, y as f32 * game_map.tile_size);
            let screen_pos = world_to_screen(world_pos, &camera_state, &rect);

            let tile_rect = Rect::from_min_size(screen_pos, Vec2::new(tile_size, tile_size));

            let color = match tile.tile_type {
                TileType::Wall => Color32::DARK_GRAY,
                TileType::SpawnPoint => Color32::BLUE,
                TileType::Empty => Color32::TRANSPARENT,
            };

            painter.rect_filled(tile_rect, 2.0, color);
        }


        // Draw player
        let player_position = self.game_data.get_field(PLAYER_POSITION).unwrap_or(Pos2::new(0.0, 0.0));
        let player_screen_pos = world_to_screen(player_position, &camera_state, &rect);
        let player_size = Vec2::new(10.0, 10.0) * camera_state.zoom;
        let player_rect = Rect::from_center_size(player_screen_pos, player_size);
        painter.rect_filled(player_rect, 0.0, Color32::RED);

        response
    }
}


fn world_to_screen(world_pos: Pos2, camera: &CameraState, rect: &Rect) -> Pos2 {
    Pos2::new(
        (world_pos.x - camera.camera_pos.x) * camera.zoom + rect.center().x,
        (world_pos.y - camera.camera_pos.y) * camera.zoom + rect.center().y,
    )
}