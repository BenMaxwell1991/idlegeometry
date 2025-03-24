use crate::game::data::game_data::GameData;
use crate::helper::lock_helper::acquire_lock;
use crate::ui::component::widget::custom_button::CustomButton;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use egui::Vec2;

pub fn show_shop(ui: &mut egui::Ui, game_data: &GameData) {
    let mut user_name = String::from("Not Loaded");
    ui.add(CustomHeading::new("Shop Coming Soon"));
    ui.separator();

    if let Some(steam_client) = acquire_lock(&game_data.steam_client, "steam_client").as_ref() {
        user_name = steam_client.friends().name();
    }

    if let Some(icon) = game_data.icons_inverted.read().unwrap().get("exit") {
        ui.add(
            CustomButton::new(
                Some(icon.clone()),
                Some("Buy 1,000 Gold ($4.99)"),
                Box::new(|| {
                println!("Button clicked by: {}", user_name)
                // initiate_purchase(steam_client.clone(), "gold_pack_1000");
            }))
                .with_size(Vec2::new(300.0, 50.0))
        );
    }
}