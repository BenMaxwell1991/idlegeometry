use crate::game::game::Game;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn show_shop(ui: &mut egui::Ui, game: Arc<Mutex<Game>>, game_clone: &Game) {
    ui.add(CustomHeading::new("Shop Coming Soon"));
    ui.separator();
}