use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::PLAYER_POSITION;
use eframe::egui::{Color32, Id, Sense, Ui, Vec2, Widget};
use egui::{Pos2, Rect, Stroke, StrokeKind};
use std::hash::Hash;
use std::ops::Add;
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
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        let available_size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::click());
        let painter = ui.painter();

        // // ✅ Capture input and store in GameData
        // let mut pressed_keys = Vec::new();
        // ui.input(|i| {
        //     if i.key_down(Key::W) { pressed_keys.push(Key::W); }
        //     if i.key_down(Key::S) { pressed_keys.push(Key::S); }
        //     if i.key_down(Key::A) { pressed_keys.push(Key::A); }
        //     if i.key_down(Key::D) { pressed_keys.push(Key::D); }
        // });
        // self.game_data.update_or_set("pressed_keys", pressed_keys.clone(), |keys| { *keys = pressed_keys.clone() });

        // Get player position (updated by GameLoop)
        let player_position = self.game_data.get_field(PLAYER_POSITION).unwrap_or(Pos2::new(100.0, 100.0)).add(rect.min.to_vec2());

        // Draw grid
        let cell_size = 40.0;
        for x in (rect.min.x as i32..rect.max.x as i32).step_by(cell_size as usize) {
            painter.vline(x as f32, rect.min.y..=rect.max.y, Stroke::new(1.0, Color32::GREEN));
        }
        for y in (rect.min.y as i32..rect.max.y as i32).step_by(cell_size as usize) {
            painter.hline(rect.min.x..=rect.max.x, y as f32, Stroke::new(1.0, Color32::GREEN));
        }

        // Draw the player
        let player_size = Vec2::new(10.0, 10.0);
        let player_rect = Rect::from_center_size(player_position, player_size);
        painter.rect_filled(player_rect, 0.0, Color32::RED);

        // Draw game area border
        painter.rect_stroke(rect, 2.0, (3.0, Color32::PURPLE), StrokeKind::Middle);

        response
    }
}
