use crate::game::game::Game;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn show_upgrades(ui: &mut egui::Ui, game: Arc<Mutex<Game>>, game_clone: &Game) {
    ui.add(CustomHeading::new("Upgrades"));
    ui.separator();
}