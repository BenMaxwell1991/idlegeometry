use crate::enums::gametab::GameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::CURRENT_TAB;
use crate::ui::component::widget::custom_button::CustomButton;
use crate::ui::component::widget::custom_heading::CustomHeading;
use egui::{Context, SidePanel};

pub fn show_side_menu(ctx: &Context, game_data: &GameData) {
    SidePanel::left("side_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            ui.add_space(5.0);
            ui.add(CustomHeading::new("Menu"));
            ui.separator();

            let buttons = vec![
                ("adventure", "Adventure", GameTab::Adventure),
                ("upgrade", "Upgrades", GameTab::Upgrades),
                ("settings", "Settings", GameTab::Settings),
                ("shop", "Shop", GameTab::Shop),
            ];

            for (icon_name, text, tab) in buttons {
                if let Some(icon) = game_data.icons_inverted.read().unwrap().get(icon_name) {
                    ui.add(CustomButton::new(
                        Some(icon.clone()),
                        Some(text),
                        Box::new(move || {
                            game_data.set_field(CURRENT_TAB, tab);
                        }),
                    ));
                    ui.separator();
                } else {
                    eprintln!("Warning: Icon '{}' not found!", icon_name);
                }
            }

            if let Some(icon) = game_data.icons_inverted.read().unwrap().get("exit") {
                ui.add(CustomButton::new(
                    Some(icon.clone()),
                    Some("Exit Game"),
                    Box::new(|| {
                        std::process::exit(0);
                    }),
                ));
            }
        });
}