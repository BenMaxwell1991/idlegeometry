use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::STEAM_CLIENT;
use crate::ui::component::widget::custom_button::CustomButton;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use egui::{TextureHandle, Vec2};
use std::collections::HashMap;
use std::sync::Arc;

pub fn show_shop(ui: &mut egui::Ui, game_data: Arc<GameData>, icons: &HashMap<String, TextureHandle>) {
    let mut user_name = String::from("Not Loaded");
    ui.add(CustomHeading::new("Shop Coming Soon"));
    ui.separator();

    if let Some(steam_client) = game_data.get_field(STEAM_CLIENT) {
        user_name = steam_client.friends().name();
    }

    if let Some(icon) = icons.get("exit") {
        ui.add(
            CustomButton::new(icon.clone(), "Buy 1,000 Gold ($4.99)", Box::new(|| {
                println!("Button clicked by: {}", user_name)
                // initiate_purchase(steam_client.clone(), "gold_pack_1000");
            }))
                .size(Vec2::new(300.0, 50.0))
        );
    }
}