use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use crate::ui::component::widget::custom_button::CustomButton;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui::{Color32, Rect, Ui, Vec2};
use eframe::emath::Align;
use egui::{CentralPanel, Layout, UiBuilder};

pub fn show_game_menu_paused(ui: &mut Ui, game_data: &GameData, game_rect: Rect) {
    let menu_rect = Rect::from_center_size(game_rect.center(), Vec2::new(300.0, 250.0));
    let painter = ui.painter();
    painter.rect_filled(menu_rect, 10.0, Color32::from_rgb(20, 20, 20));

    let buttons = vec![
        ("Resume Game", GameState::Playing),
        ("Quit", GameState::Quitting),
    ];

    CentralPanel::default().show_inside(ui, |ui| {
        ui.allocate_new_ui(
            UiBuilder::new()
                .max_rect(menu_rect)
                .layout(Layout::top_down_justified(Align::Min)),
            |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.add(CustomHeading::new("Game Menu"));
                    ui.separator();
                    ui.add_space(10.0);

                    for (text, state) in buttons {
                        ui.add(CustomButton::new(
                            None,
                            Some(text),
                            Box::new(move || {
                                game_data.set_game_state(state);
                            }),
                        ));
                        ui.separator();
                    }
                });
            },
        );
    });
}
