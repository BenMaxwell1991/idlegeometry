use crate::game::game_data::GameData;
use crate::resources::resource::Resource;
use crate::ui::component::widget::custom_grid::CustomGrid;
use crate::ui::component::widget::custom_heading::CustomHeading;
use crate::ui::component::widget::custom_progress_bar::CustomProgressBar;
use crate::ui::component::widget::game_graphics::GameGraphics;
use eframe::egui;
use eframe::egui::{Align, Layout};
use egui::Vec2;
use std::sync::{Arc, OnceLock};
use uuid::Uuid;

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
            columns[0].with_layout(Layout::top_down(Align::Min), |ui| {
                ui.add(CustomGrid::new(game_data_one, grid_id));
                ui.add_space(ui.available_height() - 30.0);

                let points = game_data.get_field::<Vec<Resource>>("resources")
                    .unwrap().iter()
                    .find(|resource| resource.name == "Points")
                    .cloned();

                if let Some(points) = points {
                    ui.add_sized(Vec2::new(ui.available_width(), 30.0), CustomProgressBar::new(points).show_percentage().set_on_click(Box::new(|| { println!("Button Clicked") })));
                } else {
                    println!("No points found");
                }
            });

            columns[1].add(GameGraphics::new(game_data_two, graphics_id));
        });
    });

}