use crate::game::game::Game;
use eframe::egui::{Color32, Id, Sense, Ui, Widget};
use std::sync::{Arc, Mutex};

pub struct GameGraphics {
    game: Arc<Mutex<Game>>,
    id: Id,
}

impl GameGraphics {
    pub fn new(game: Arc<Mutex<Game>>, id_salt: impl std::hash::Hash) -> Self {
        Self {
            game,
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
