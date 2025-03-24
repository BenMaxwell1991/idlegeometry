use eframe::egui;
use eframe::egui::{Color32, FontId, Pos2, Response, Ui, Vec2, Widget};
use eframe::epaint::FontFamily;
use egui::Sense;
use crate::ui::asset::loader::DP_COMIC_FONT;

pub struct LabelNoInteract<'a> {
    pub text: &'a str,
    pub font: String,
    pub font_size: f32,
    pub colour: Color32,
    pub shadow_colour: Option<Color32>,
    pub shadow_offset: Option<Vec2>,
    pub text_wrapped: bool,
}

impl<'a> LabelNoInteract<'a> {
    pub fn new(text: &'a str, font: String, font_size: f32, colour: Color32) -> Self {
        Self { text, font, font_size, colour, shadow_colour: None, shadow_offset: None, text_wrapped: false }
    }

    pub fn with_shadow(mut self, shadow_colour: Color32, shadow_offset: Vec2) -> Self {
        self.shadow_colour = Some(shadow_colour);
        self.shadow_offset = Some(shadow_offset);
        self
    }

    pub fn with_wrapping(mut self, text_wrapped: bool) -> Self {
        self.text_wrapped = text_wrapped;
        self
    }
}

impl<'a> Widget for LabelNoInteract<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let font_id = FontId::new(self.font_size, FontFamily::Name(self.font.clone().into()));

        let galley = ui.fonts(|fonts| {
            if self.text_wrapped {
                let wrap_width = ui.available_width();
                fonts.layout(self.text.to_string(), font_id.clone(), self.colour, wrap_width)
            } else {
                fonts.layout_no_wrap(self.text.to_string(), font_id.clone(), self.colour)
            }
        });

        let desired_size = galley.size();
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());
        let text_pos = Pos2::new(rect.min.x, rect.min.y);

        if let (Some(shadow_colour), Some(shadow_offset)) = (self.shadow_colour, self.shadow_offset) {
            let galley_shadow = ui.fonts(|fonts| {
                if self.text_wrapped {
                    let wrap_width = ui.available_width();
                    fonts.layout(self.text.to_string(), font_id.clone(), shadow_colour, wrap_width)
                } else {
                    fonts.layout_no_wrap(self.text.to_string(), font_id.clone(), shadow_colour)
                }
            });
            ui.painter().galley(text_pos + shadow_offset, galley_shadow, shadow_colour);
        }

        ui.painter().galley(text_pos, galley, self.colour);
        response
    }
}
