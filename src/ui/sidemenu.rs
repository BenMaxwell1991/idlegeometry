use crate::enums::gametab::GameTab;
use crate::game::game::Game;
use crate::ui::component::menu_button::MenuButton;
use eframe::egui;
use eframe::egui::TextureHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::ui::component::custom_button::CustomButton;

pub fn show_side_menu(ctx: &egui::Context, game: Arc<Mutex<Game>>, icons: &HashMap<String, TextureHandle>) {
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            ui.heading("📌 Menu");
            ui.separator();

            if let Ok(mut game) = game.lock() {
                let buttons = vec![
                    ("geometry", "Geometry", GameTab::Geometry),
                    ("upgrade", "Upgrades", GameTab::Upgrades),
                    ("settings", "Settings", GameTab::Settings),
                    ("shop", "Shop", GameTab::Shop),
                ];

                for (icon_name, text, tab) in buttons {
                    if let Some(icon) = icons.get(icon_name) {
                        let game_ref = &mut game;
                        CustomButton::new(
                            icon.clone(),
                            text,
                            Box::new(move || {
                                game_ref.current_tab = tab;
                            })).show(ui);
                    } else {
                        eprintln!("Warning: Icon '{}' not found!", icon_name);
                    }
                }

                ui.separator();

                if let Some(icon) = icons.get("shop") {
                    MenuButton {
                        text: "Exit Game",
                        icon: icon.clone(),
                        on_click: Box::new(|| {
                            std::process::exit(0);
                        }),
                    }
                        .show(ui);
                }
            }
        });
}
