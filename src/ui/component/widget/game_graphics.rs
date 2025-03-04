use crate::game::game::Game;
use eframe::egui::{Color32, Id, Sense, Ui, Widget};
use std::sync::{Arc, Mutex};
use crate::game::game_data::GameData;

pub struct GameGraphics {
    game_data: Arc<GameData>,
    id: Id,
}

impl GameGraphics {
    pub fn new(game_data: Arc<GameData>, id_salt: impl std::hash::Hash) -> Self {
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
        let center = rect.center();

        painter.rect_filled(rect, 0.0, Color32::WHITE);
        let time = ui.input(|i| i.time);
        let radius = 50.0 + (time.sin() * 20.0) as f32;

        painter.circle_filled(center, radius, Color32::from_rgb(200, 50, 50));

        response
    }
}
