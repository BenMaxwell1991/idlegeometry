use crate::game::game::{Game, GameTab};
use eframe::egui;
use std::sync::{Arc, Mutex};
// ✅ Import GameTab

pub fn show_side_panel(ctx: &egui::Context, game: Arc<Mutex<Game>>) {
    egui::SidePanel::left("side_panel") // ✅ Keep the sidebar
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            ui.heading("📌 Menu");
            ui.separator();

            if let Ok(mut game) = game.lock() {
                // ✅ Button to switch to Geometry (detects on mouse down)
                let geom_response = ui.add(egui::Button::new("📊 Geometry"));
                if geom_response.clicked() || geom_response.interact(egui::Sense::click()).is_pointer_button_down_on() {
                    game.current_tab = GameTab::Geometry;
                }

                // ✅ Button to switch to Settings (detects on mouse down)
                let settings_response = ui.add(egui::Button::new("⚙ Settings"));
                if settings_response.clicked() || settings_response.interact(egui::Sense::click()).is_pointer_button_down_on() {
                    game.current_tab = GameTab::Settings;
                }

                ui.separator();

                // ✅ Exit Game Button (still uses clicked())
                if ui.button("❌ Exit Game").clicked() {
                    std::process::exit(0);
                }
            }
        });
}
