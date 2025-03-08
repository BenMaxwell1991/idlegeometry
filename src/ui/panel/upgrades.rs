use crate::game::data::game_data::GameData;
use crate::ui::component::widget::custom_heading::CustomHeading;
use egui::Ui;
use std::sync::Arc;

pub fn show_upgrades(ui: &mut Ui, game_data: Arc<GameData>) {
    ui.add(CustomHeading::new("Upgrades"));
    ui.separator();
}