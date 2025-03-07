use crate::enums::gametab::GameTab;
use crate::game::game_data::GameData;
use crate::ui::component::widget::custom_button::CustomButton;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use eframe::egui::TextureHandle;
use std::collections::HashMap;
use std::sync::Arc;

pub fn show_side_menu(ctx: &egui::Context, game_data: Arc<GameData>, icons: &HashMap<String, TextureHandle>) {
    egui::SidePanel::left("side_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            ui.add(CustomHeading::new("Menu"));
            ui.separator();

            let buttons = vec![
                ("geometry", "Geometry", GameTab::Geometry),
                ("upgrade", "Upgrades", GameTab::Upgrades),
                ("settings", "Settings", GameTab::Settings),
                ("shop", "Shop", GameTab::Shop),
            ];

            for (icon_name, text, tab) in buttons {
                let game_data_clone = Arc::clone(&game_data);
                if let Some(icon) = icons.get(icon_name) {
                    ui.add(CustomButton::new(
                        icon.clone(),
                        text,
                        Box::new(move || {
                            game_data_clone.set_field("current_tab", tab);
                        }),
                    ));
                    ui.separator();
                } else {
                    eprintln!("Warning: Icon '{}' not found!", icon_name);
                }
            }

            if let Some(icon) = icons.get("exit") {
                ui.add(CustomButton::new(
                    icon.clone(),
                    "Exit Game",
                    Box::new(|| {
                        std::process::exit(0);
                    }),
                ));
            }
        });
}