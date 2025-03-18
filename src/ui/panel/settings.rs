use crate::enums::numberformatmode::NumberFormatMode;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::SETTINGS;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use eframe::egui::{Align, ComboBox, Layout, Slider};
use std::sync::OnceLock;
use uuid::Uuid;

static COMBOBOX_ID: OnceLock<Uuid> = OnceLock::new();
static RESOLUTION_ID: OnceLock<Uuid> = OnceLock::new();

pub fn show_settings_panel(ui: &mut egui::Ui, game_data: &GameData) {
    ui.add(CustomHeading::new("Settings Panel"));
    ui.separator();

    let combobox_id = *COMBOBOX_ID.get_or_init(Uuid::new_v4);
    let resolution_id = *RESOLUTION_ID.get_or_init(Uuid::new_v4);

    ui.with_layout(Layout::top_down(Align::Min), |ui| {
        let settings = game_data.get_field(SETTINGS).unwrap_or_default();

        ui.horizontal(|ui| {
            ui.label("Number Format Mode:");
            ComboBox::from_id_salt(combobox_id)
                .selected_text(format!("{:?}", settings.number_format_mode))
                .show_ui(ui, |ui| {
                    if ui.selectable_label(settings.number_format_mode == NumberFormatMode::Standard, "Standard").clicked() {
                        let mut updated_settings = settings;
                        updated_settings.number_format_mode = NumberFormatMode::Standard;
                        game_data.set_field(SETTINGS, updated_settings);
                    }
                    if ui.selectable_label(settings.number_format_mode == NumberFormatMode::Engineering, "Engineering").clicked() {
                        let mut updated_settings = settings;
                        updated_settings.number_format_mode = NumberFormatMode::Engineering;
                        game_data.set_field(SETTINGS, updated_settings);
                    }
                    if ui.selectable_label(settings.number_format_mode == NumberFormatMode::Exponential, "Exponential").clicked() {
                        let mut updated_settings = settings;
                        updated_settings.number_format_mode = NumberFormatMode::Exponential;
                        game_data.set_field(SETTINGS, updated_settings);
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("Resolution:");
            ComboBox::from_id_salt(resolution_id)
                .selected_text(format!("{} x {}", settings.window_width, settings.window_height))
                .show_ui(ui, |ui| {
                    if ui.selectable_label(settings.window_width == 1920.0, "1920 x 1080").clicked() {
                        let mut updated_settings = settings;
                        updated_settings.window_width = 1920.0;
                        updated_settings.window_height = 1080.0;
                        game_data.set_field(SETTINGS, updated_settings);
                    }
                    if ui.selectable_label(settings.window_width == 1280.0, "1280 x 720").clicked() {
                        let mut updated_settings = settings;
                        updated_settings.window_width = 1280.0;
                        updated_settings.window_height = 720.0;
                        game_data.set_field(SETTINGS, updated_settings);
                    }
                    if ui.selectable_label(settings.window_width == 800.0, "800 x 600").clicked() {
                        let mut updated_settings = settings;
                        updated_settings.window_width = 800.0;
                        updated_settings.window_height = 600.0;
                        game_data.set_field(SETTINGS, updated_settings);
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("V-Sync:");
            let mut vsync = settings.vsync;
            if ui.checkbox(&mut vsync, "Enabled").changed() {
                let mut updated_settings = settings;
                updated_settings.vsync = vsync;
                game_data.set_field(SETTINGS, updated_settings);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Auto-Save Interval (Seconds):");
            let mut autosave_interval = settings.autosave_interval;
            if ui.add(Slider::new(&mut autosave_interval, 1..=60).text("Seconds")).changed() {
                let mut updated_settings = settings;
                updated_settings.autosave_interval = autosave_interval;
                game_data.set_field(SETTINGS, updated_settings);
            }
        });
    });
}
