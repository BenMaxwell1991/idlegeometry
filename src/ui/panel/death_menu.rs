use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use crate::ui::component::widget::custom_button::CustomButton;
use crate::ui::component::widget::custom_heading::CustomHeading;
use eframe::egui::{Color32, Rect, Ui, Vec2};
use eframe::emath::Align;
use egui::{Layout, Stroke, StrokeKind, UiBuilder};

pub fn show_death_menu(ui: &mut Ui, game_data: &GameData, game_rect: Rect) {
    let menu_rect = Rect::from_center_size(game_rect.center(), Vec2::new(320.0, 270.0));
    let painter = ui.painter();
    painter.rect_filled(menu_rect, 10.0, Color32::from_rgb(20, 20, 20));
    painter.rect_stroke(menu_rect, 10.0, Stroke::new(1.5, Color32::WHITE), StrokeKind::Inside); // optional border

    let buttons = vec![
        ("Return to Lair", GameState::Lair),
        ("Quit", GameState::Quitting),
    ];

    let mut heading = CustomHeading::new("DEFEAT!!");
    heading.font_size = 60.0;
    heading.font_colour = Color32::RED;

    ui.allocate_new_ui(
        UiBuilder::new()
            .max_rect(menu_rect)
            .layout(Layout::top_down(Align::Center)),
        |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(25.0);
                ui.add(heading);
                ui.separator();
                ui.add_space(10.0);

                for (text, state) in buttons {
                    ui.add(CustomButton::new(
                        None,
                        Some(text),
                        Box::new({
                            let game_data = game_data.clone();
                            move || {
                                game_data.set_game_state(state);
                            }
                        }),
                    ));
                    ui.add_space(6.0);
                }
            });
        },
    );
}
