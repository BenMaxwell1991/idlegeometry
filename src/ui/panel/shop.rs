use crate::game::game_data::GameData;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use std::sync::Arc;

pub fn show_shop(ui: &mut egui::Ui, game_data: Arc<GameData>) {
    ui.add(CustomHeading::new("Shop Coming Soon"));
    ui.separator();
}