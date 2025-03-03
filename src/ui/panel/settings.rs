use crate::enums::numberformatmode::NumberFormatMode;
use crate::game::game::Game;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use eframe::egui::{Align, ComboBox, Layout, Slider};
use std::sync::{Arc, Mutex, OnceLock};
use uuid::Uuid;

static COMBOBOX_ID: OnceLock<Uuid> = OnceLock::new();
static RESOLUTION_ID: OnceLock<Uuid> = OnceLock::new();

pub fn show_settings_panel(ui: &mut egui::Ui, game: Arc<Mutex<Game>>) {
    ui.add(CustomHeading::new("Settings Panel"));
    ui.separator();

    let combobox_id = *COMBOBOX_ID.get_or_init(Uuid::new_v4);
    let resolution_id = *RESOLUTION_ID.get_or_init(Uuid::new_v4);

    ui.with_layout(Layout::top_down(Align::Min), |ui| {
        if let Ok(mut game) = game.lock() {
            // 📌 Number Format Mode Dropdown
            ui.horizontal(|ui| {
                ui.label("Number Format Mode:");
                ComboBox::from_id_salt(combobox_id)
                    .selected_text(format!("{:?}", game.settings.number_format_mode))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut game.settings.number_format_mode, NumberFormatMode::Standard, "Standard");
                        ui.selectable_value(&mut game.settings.number_format_mode, NumberFormatMode::Engineering, "Engineering");
                        ui.selectable_value(&mut game.settings.number_format_mode, NumberFormatMode::Exponential, "Exponential");
                    });
            });

            // 📌 Resolution Dropdown
            ui.horizontal(|ui| {
                ui.label("Resolution:");
                ComboBox::from_id_salt(resolution_id)
                    .selected_text(format!("{} x {}", game.settings.window_width, game.settings.window_height))
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(game.settings.window_width == 1920.0, "1920 x 1080").clicked() {
                            game.settings.window_width = 1920.0;
                            game.settings.window_height = 1080.0;
                        }
                        if ui.selectable_label(game.settings.window_width == 1280.0, "1280 x 720").clicked() {
                            game.settings.window_width = 1280.0;
                            game.settings.window_height = 720.0;
                        }
                        if ui.selectable_label(game.settings.window_width == 800.0, "800 x 600").clicked() {
                            game.settings.window_width = 800.0;
                            game.settings.window_height = 600.0;
                        }
                    });
            });

            // 📌 V-Sync Toggle
            ui.horizontal(|ui| {
                ui.label("V-Sync:");
                ui.checkbox(&mut game.settings.vsync, "Enabled");
            });

            // 📌 Auto-Save Interval Input
            ui.horizontal(|ui| {
                ui.label("Auto-Save Interval (Seconds):");
                ui.add(Slider::new(&mut game.settings.autosave_interval, 1..=60).text("Seconds"));
            });
        }
    });
}
