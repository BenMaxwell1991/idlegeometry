use crate::game::game::Game;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn show_geometry(ui: &mut egui::Ui, game: Arc<Mutex<Game>>) {
    ui.heading("📊 Resource Dashboard");

    if let Ok(game) = game.lock() {
        egui::Grid::new("resource_grid")
            .striped(true)
            .show(ui, |ui| {
                for resource in &game.resources {
                    if resource.unlocked {
                        ui.label(format!("{}:", resource.name));
                        ui.label(&resource.amount.format_number(game.settings.number_format_mode));
                        ui.end_row();
                    }
                }
            });
    }
}