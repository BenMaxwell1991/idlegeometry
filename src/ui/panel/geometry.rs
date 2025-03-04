use crate::game::game::Game;
use crate::ui::component::widget::custom_grid::CustomGrid;
use crate::ui::component::widget::custom_heading::CustomHeading;
use crate::ui::component::widget::game_graphics::GameGraphics;
use eframe::egui;
use eframe::egui::{Align, Layout};
use std::sync::{Arc, Mutex, OnceLock};
use uuid::Uuid;
use crate::game::game_data::GameData;

static RESOURCE_GRID_ID: OnceLock<Uuid> = OnceLock::new();
static GAME_GRAPHICS_ID: OnceLock<Uuid> = OnceLock::new();

pub fn show_geometry(ui: &mut egui::Ui, game_data: Arc<GameData>) {
    let grid_id = RESOURCE_GRID_ID.get_or_init(Uuid::new_v4);
    let graphics_id = GAME_GRAPHICS_ID.get_or_init(Uuid::new_v4);

    let game_data_one = Arc::clone(&game_data);
    let game_data_two = Arc::clone(&game_data);

    ui.add(CustomHeading::new("1D Research"));
    ui.separator();
    ui.add_space(10.0);
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        ui.columns(2, |columns| {
            columns[0].add(CustomGrid::new(game_data_one, grid_id));
            columns[1].add(GameGraphics::new(game_data_two, graphics_id));
        });
    });

}