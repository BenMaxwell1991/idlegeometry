use std::string::ToString;
use crate::ui::component::widget::label_no_interact::LabelNoInteract;
use eframe::egui::{Color32, Response, Stroke, StrokeKind, Ui, Vec2, Widget};
use egui::Sense;
use crate::ui::asset::loader::DP_COMIC_FONT;

const HEADING_SIZE: Vec2 = Vec2::new(200.0, 50.0);
const BACKGROUND_COLOUR: Option<Color32> = None;
const BORDER_COLOUR: Option<Color32> = None;
const BORDER_WIDTH: f32 = 1.0;
const FONT_DEFAULT: &str = DP_COMIC_FONT;
const FONT_SIZE: f32 = 50.0;
const FONT_COLOUR: Color32 = Color32::WHITE;
const SHADOW_COLOUR: Color32 = Color32::from_rgba_premultiplied(100, 0, 100, 50);
const SHADOW_OFFSET: Vec2 = Vec2::new(3.0, 3.0);

pub struct CustomHeading<'a> {
    pub text: &'a str,
    pub size: Vec2,
    pub border_thickness: f32,
    pub border_colour: Option<Color32>,
    pub background_colour: Option<Color32>,
    pub font: String,
    pub font_size: f32,
    pub font_colour: Color32,
    pub shadow_colour: Color32,
    pub shadow_offset: Vec2,
    pub text_wrapping: bool,
}

impl<'a> CustomHeading<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            size: HEADING_SIZE,
            border_thickness: BORDER_WIDTH,
            border_colour: BORDER_COLOUR,
            background_colour: BACKGROUND_COLOUR,
            font: FONT_DEFAULT.to_string(),
            font_size: FONT_SIZE,
            font_colour: FONT_COLOUR,
            shadow_colour: SHADOW_COLOUR,
            shadow_offset: SHADOW_OFFSET,
            text_wrapping: false,
        }
    }
}

impl<'a> Widget for CustomHeading<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());

        if let Some(background_colour) = self.background_colour {
            ui.painter().rect_filled(rect, 5.0, background_colour);
        }

        if let Some(border_colour) = self.border_colour {
            ui.painter().rect_stroke(
                rect,
                5.0,
                Stroke::new(self.border_thickness, border_colour),
                StrokeKind::Inside,
            );
        }

        let label = LabelNoInteract::new(self.text, self.font, self.font_size, self.font_colour).with_shadow(self.shadow_colour, self.shadow_offset).with_wrapping(self.text_wrapping);
        ui.put(rect, label);

        response
    }
}