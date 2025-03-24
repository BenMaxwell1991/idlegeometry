use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use crate::game::data::initialise_adventure::initialise_adventure;
use crate::ui::component::widget::custom_button::CustomButton;
use eframe::egui::{Rect, Ui, Vec2};
use eframe::emath::Align;
use egui::{Layout, Pos2, UiBuilder};

pub fn show_begin_adventure(ui: &mut Ui, game_data: &GameData, game_rect: Rect) {
    let button_size = Vec2::new(250.0, 50.0);
    let bottom_center = Pos2::new(
        game_rect.center().x - button_size.x / 2.0,
        game_rect.max.y - button_size.y - 70.0,
    );
    let button_rect = Rect::from_min_size(bottom_center, button_size);

    let begin_adventure_button = CustomButton::new(
        None,
        Some("Begin Adventure"),
        Box::new({
            let game_data = game_data.clone();
            move || {
                initialise_adventure(&game_data);
                game_data.set_game_state(GameState::Playing);
            }
        }),
    ).with_size(button_size);

    ui.allocate_new_ui(
        UiBuilder::new()
            .max_rect(button_rect)
            .layout(Layout::top_down_justified(Align::Min)),
        |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.add(begin_adventure_button);
            });
        },
    );
}
