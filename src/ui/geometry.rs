use crate::game::game::Game;
use crate::ui::component::widget::custom_grid::CustomGrid;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui;
use eframe::egui::{Color32, Layout, Vec2};
use std::sync::{Arc, Mutex};

const RESOURCE_TEXT_COLOUR: Color32 = Color32::from_rgb(255, 255, 255);

pub fn show_geometry(ui: &mut egui::Ui, game: Arc<Mutex<Game>>) {
    ui.add(CustomHeading::new("1D Research"));
    ui.separator();

    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), 0.0),
        Layout::centered_and_justified(egui::Direction::LeftToRight),
        |ui| {
            ui.add(CustomGrid::new(game, "ResourceGrid"));
        },
    );
}