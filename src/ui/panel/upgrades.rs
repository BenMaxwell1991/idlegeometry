use crate::game::data::game_data::GameData;
use crate::ui::component::widget::custom_heading::CustomHeading;
use egui::Ui;

pub fn show_upgrades(ui: &mut Ui, game_data: &GameData) {
    ui.add(CustomHeading::new("Upgrades"));
    ui.separator();
}