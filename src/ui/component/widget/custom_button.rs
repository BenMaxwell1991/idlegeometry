use crate::ui::asset::loader::DP_COMIC_FONT;
use crate::ui::component::widget::interactive_widget::InteractiveWidget;
use crate::ui::component::widget::label_no_interact::LabelNoInteract;
use eframe::egui::load::SizedTexture;
use eframe::egui::{
    Align, Color32, Direction, FontId, Image, Layout, Response, Sense, Stroke, StrokeKind,
    TextureHandle, Ui, UiBuilder, Vec2, Widget,
};
use eframe::emath::{Pos2, Rect};
use eframe::epaint::FontFamily;
use std::string::ToString;

const BUTTON_SIZE: Vec2 = Vec2::new(200.0, 50.0);
const TEXT_COLOUR: Color32 = Color32::WHITE;
const BACKGROUND_COLOUR: Color32 = Color32::BLACK;
const BORDER_COLOUR: Color32 = Color32::from_rgb(100, 0, 100);
const BORDER_WIDTH: f32 = 2.0;
const ICON_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const GAP_SIZE: f32 = 5.0;
const FONT_DEFAULT: &str = DP_COMIC_FONT;
const FONT_SIZE: f32 = 32.0;

pub struct CustomButton<'a> {
    pub icon: Option<TextureHandle>,
    pub text: Option<&'a str>,
    pub on_click: Box<dyn FnMut() + 'a>,
    pub size: Vec2,
    pub border_thickness: f32,
    pub border_colour: Color32,
    pub background_colour: Color32,
    pub icon_size: Vec2,
    pub gap_size: f32,
    pub font: String,
    pub font_size: f32,
    pub align: Align,
}

impl<'a> CustomButton<'a> {
    pub fn new(
        icon: Option<TextureHandle>,
        text: Option<&'a str>,
        on_click: Box<dyn FnMut() + 'a>
    ) -> Self {
        Self {
            icon,
            text,
            on_click,
            size: BUTTON_SIZE,
            border_thickness: BORDER_WIDTH,
            border_colour: BORDER_COLOUR,
            background_colour: BACKGROUND_COLOUR,
            icon_size: ICON_SIZE,
            gap_size: GAP_SIZE,
            font: FONT_DEFAULT.to_string(),
            font_size: FONT_SIZE,
            align: Align::Center,
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}

impl<'a> InteractiveWidget for CustomButton<'a> {
    fn click_colours(&self) -> Vec<i8> {
        vec![-25, -30]
    }
    fn hover_colours(&self) -> Vec<i8> {
        vec![20, 30]
    }
}

impl<'a> Widget for CustomButton<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let parent_rect = ui.available_rect_before_wrap();
        let mut rect = ui.allocate_exact_size(self.size, Sense::empty()).0;

        if self.align == Align::Center {
            rect = Rect::from_min_size(Pos2::new(parent_rect.center().x - self.size.x / 2.0, rect.min.y), self.size);
        }

        let response = ui.interact(rect, ui.make_persistent_id(self.text.unwrap_or("button")), Sense::click());

        let button_clicked = self.button_clicked(ui, &response);
        let click_released_inside = self.released_inside(ui, &response);
        let base_colours = vec![self.background_colour, self.border_colour];
        let [background_colour, border_colour]: [Color32; 2] =
            self.determine_colour(base_colours, button_clicked, response.contains_pointer())
                .try_into()
                .expect("Invalid Array Size");

        if click_released_inside {
            (self.on_click)();
        }

        let painter = ui.painter();
        painter.rect_filled(rect, 5.0, background_colour);
        painter.rect_stroke(rect, 5.0, Stroke::new(self.border_thickness, border_colour), StrokeKind::Inside);

        ui.allocate_new_ui(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
            |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(self.calculate_padding(&ui).max(0.0));

                    if let Some(icon) = &self.icon {
                        ui.add(Image::new(SizedTexture::new(icon.id(), self.icon_size)));
                        ui.add_space(self.gap_size);
                    }

                    if let Some(text) = self.text {
                        ui.add(LabelNoInteract::new(text, self.font.clone(), self.font_size, TEXT_COLOUR));
                    }

                    ui.add_space(self.calculate_padding(&ui).max(0.0));
                });
            },
        );

        response
    }
}

impl<'a> CustomButton<'a> {
    fn calculate_padding(&mut self, ui: &Ui) -> f32 {
        let font_id = FontId::new(self.font_size, FontFamily::Name(self.font.clone().into()));

        let label_width = if let Some(text) = self.text {
            ui.fonts(|f| {
                f.layout_no_wrap(text.to_string(), font_id, TEXT_COLOUR)
                    .size()
                    .x
            })
        } else {
            0.0
        };

        let icon_width = if self.icon.is_some() {
            self.icon_size.x + self.gap_size
        } else {
            0.0
        };

        let desired_width = icon_width + label_width;
        let available_width = ui.available_width();

        (available_width - desired_width).max(0.0) / 2.0
    }
}