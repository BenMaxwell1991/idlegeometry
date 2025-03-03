use eframe::egui;
use eframe::egui::{Color32, FontId, Pos2, Response, Ui, Vec2, Widget};

pub struct LabelNoInteract<'a> {
    pub text: &'a str,
    pub font_size: f32,
    pub colour: Color32,
    pub shadow_colour: Option<Color32>,
    pub shadow_offset: Option<Vec2>,
}

impl<'a> LabelNoInteract<'a> {
    pub fn new(text: &'a str, font_size: f32, colour: Color32) -> Self {
        Self { text, font_size, colour, shadow_colour: None, shadow_offset: None }
    }

    pub fn with_shadow(mut self, shadow_colour: Color32, shadow_offset: Vec2) -> Self {
        self.shadow_colour = Some(shadow_colour);
        self.shadow_offset = Some(shadow_offset);
        self
    }
}

impl<'a> Widget for LabelNoInteract<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let font_id = FontId::proportional(self.font_size);
        let text_size = ui.fonts(|f| f.layout_no_wrap(self.text.to_string(), font_id.clone(), self.colour).size());
        let (rect, response) = ui.allocate_exact_size(text_size, egui::Sense::hover());
        let text_pos = Pos2::new(rect.min.x, rect.center().y);

        match (self.shadow_colour, self.shadow_offset) {
            (Some(shadow_colour), Some(shadow_offset)) => {
                ui.painter().text(
                    text_pos + shadow_offset,
                    egui::Align2::LEFT_CENTER,
                    self.text,
                    font_id.clone(),
                    shadow_colour
                );
            }
            _ => {}
        }

        ui.painter().text(text_pos, egui::Align2::LEFT_CENTER, self.text, font_id, self.colour);
        response
    }
}
