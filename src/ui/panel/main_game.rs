use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::RESOURCES;
use crate::ui::component::widget::custom_grid::CustomGrid;
use crate::ui::component::widget::custom_heading::CustomHeading;
use crate::ui::component::widget::custom_progress_bar::CustomProgressBar;
use crate::ui::component::widget::game_graphics::GameGraphics;
use crate::ui::panel::game_menu::show_game_menu;
use eframe::{egui, Frame};
use egui::{Color32, Pos2, Rect, Ui, Vec2};
use std::sync::OnceLock;
use uuid::Uuid;

static GAME_GRAPHICS_ID: OnceLock<Uuid> = OnceLock::new();
static RESOURCE_HUD_ID: OnceLock<Uuid> = OnceLock::new();

pub fn show_main_game(ui: &mut Ui, game_data: &GameData, frame: &mut Frame) {
    let game_state = *game_data.game_state.read().unwrap();
    let hud_id = RESOURCE_HUD_ID.get_or_init(Uuid::new_v4);

    ui.add(CustomHeading::new("The Game"));
    ui.separator();

    let game_rect = ui.available_rect_before_wrap();
    let (hud_rect, progress_rect) = get_hud_rects(&game_rect);

    match game_state {
        GameState::Ready => {
            show_game_menu(ui, game_data, game_rect);
        }
        GameState::Playing => {
            ui.put(game_rect, GameGraphics::new(game_data, frame));
        }
        _ => {}
    }

    let painter = ui.painter();
    painter.rect_filled(hud_rect, 10.0, Color32::from_rgb(50, 0, 50));
    painter.rect_filled(progress_rect, 5.0, Color32::from_rgb(50, 0, 50));

    ui.put(hud_rect, CustomGrid::new(game_data, hud_id));

    if let Some(points) = game_data.get_field(RESOURCES)
        .unwrap().iter()
        .find(|resource| resource.name == "Points")
        .cloned()
    {
        ui.put(progress_rect, CustomProgressBar::new(points)
            .show_percentage()
            .set_on_click(Box::new(|| { println!("Progress Bar Clicked") }))
        );
    }
}

fn get_hud_rects(game_rect: &Rect) -> (Rect, Rect) {
    let hud_size = Vec2::new(300.0, 22.0);
    let hud_pos = Pos2::new(game_rect.min.x + 20.0, game_rect.min.y + 20.0);
    let hud_rect = Rect::from_min_size(hud_pos, hud_size);

    let progress_size = Vec2::new(400.0, 30.0);
    let progress_pos = Pos2::new(
        game_rect.center().x - progress_size.x / 2.0,
        game_rect.max.y - 60.0
    );
    let progress_rect = Rect::from_min_size(progress_pos, progress_size);

    (hud_rect, progress_rect)
}