use crate::enums::gametab::GameTab;
use crate::game::game::Game;
use crate::ui::component::widget::custom_button::CustomButton;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use eframe::egui::TextureHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn show_side_menu(ctx: &egui::Context, game: Arc<Mutex<Game>>, icons: &HashMap<String, TextureHandle>) {
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            ui.add(CustomHeading::new("Menu"));
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
                        ui.add(CustomButton::new(
                            icon.clone(),
                            text,
                            Box::new(move || {
                                game_ref.current_tab = tab;
                            })));
                        ui.separator();
                    } else {
                        eprintln!("Warning: Icon '{}' not found!", icon_name);
                    }
                }

                if let Some(icon) = icons.get("exit") {
                    ui.add(CustomButton::new(
                        icon.clone(),
                        "Exit Game",
                        Box::new(move || {
                            std::process::exit(0);
                        })));
                }
            }
        });
}
