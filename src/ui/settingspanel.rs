use crate::enums::numberformatmode::NumberFormatMode;
use crate::game::game::Game;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn show_settings_panel(ui: &mut egui::Ui, game: Arc<Mutex<Game>>) {
    ui.heading("⚙ Settings Panel");
    ui.separator();

    if let Ok(mut game) = game.lock() {
        ui.label("Number Format Mode:");

        egui::ComboBox::from_label("Format Mode")
            .selected_text(format!("{:?}", game.settings.number_format_mode))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut game.settings.number_format_mode, NumberFormatMode::Standard, "Standard");
                ui.selectable_value(&mut game.settings.number_format_mode, NumberFormatMode::Scientific, "Scientific");
                ui.selectable_value(&mut game.settings.number_format_mode, NumberFormatMode::Exponential, "Exponential");
            });
    }
}
