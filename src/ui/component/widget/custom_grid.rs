use crate::game::game::Game;
use crate::ui::helper::layout_helper::centered_ui;
use eframe::egui::{Color32, FontId, Grid, Id, RichText, Sense, Ui, Vec2, Widget};

const RESOURCE_TEXT_COLOUR: Color32 = Color32::from_rgb(255, 255, 255);
const COLUMN_SPACING: f32 = 20.0;

pub struct CustomGrid<'a> {
    game: &'a Game,
    id: Id,
}

impl<'a> CustomGrid<'a> {
    pub fn new(game_clone: &'a Game, id_salt: impl std::hash::Hash) -> Self {
        Self {
            game: game_clone,
            id: Id::new(id_salt),
        }
    }
}

impl<'a> Widget for CustomGrid<'a> {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        let longest_row = longest_row(ui, &self.game);
        centered_ui(ui, longest_row, |ui| {
            Grid::new(self.id)
                .show(ui, |ui| {
                    for resource in &self.game.resources {
                        if resource.unlocked {
                            ui.label(RichText::new(format!("{}:", resource.name)).color(RESOURCE_TEXT_COLOUR));
                            ui.allocate_exact_size(Vec2::new(COLUMN_SPACING, 1.0), Sense::hover());
                            ui.label(RichText::new(format!("{}", resource.amount.format_number(self.game.settings.number_format_mode))).color(RESOURCE_TEXT_COLOUR));
                            ui.allocate_exact_size(Vec2::new(COLUMN_SPACING, 1.0), Sense::hover());
                            ui.label(RichText::new(format!("{}/s", resource.rate.format_number(self.game.settings.number_format_mode))).color(RESOURCE_TEXT_COLOUR));
                            ui.end_row();
                        }
                    }
                })
        }).response
    }
}

fn text_width(ui: &Ui, text: &str, font_size: f32) -> f32 {
    let font_id = FontId::proportional(font_size);
    ui.fonts(|f| f.layout_no_wrap(text.to_string(), font_id, RESOURCE_TEXT_COLOUR).size().x)
}

fn longest_row(ui: &mut Ui, game: &Game) -> f32 {
    let mut longest_row_width = 0.0;

    for resource in &game.resources {
        if resource.unlocked {
            let name_text = format!("{}:", resource.name);
            let amount_text = format!("{}", resource.amount.format_number(game.settings.number_format_mode));
            let rate_text = format!("{}/s", resource.rate.format_number(game.settings.number_format_mode));

            let name_width = text_width(ui, &name_text, 18.0);
            let amount_width = text_width(ui, &amount_text, 18.0);
            let rate_width = text_width(ui, &rate_text, 18.0);

            let row_width = name_width + amount_width + rate_width + COLUMN_SPACING * 2.0;
            if row_width > longest_row_width {
                longest_row_width = row_width;
            }
        }
    }

    longest_row_width
}