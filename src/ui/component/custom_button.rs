use eframe::egui::{Color32, Direction, Image, Label, Layout, Response, RichText, Sense, Stroke, StrokeKind, TextureHandle, Ui, UiBuilder, Vec2};
use eframe::egui::load::SizedTexture;

const BUTTON_SIZE: Vec2 = Vec2::new(130.0, 40.0); // Custom button size
const IMAGE_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const TEXT_COLOUR: Color32 = Color32::WHITE;
const BACKGROUND_COLOUR: Color32 = Color32::BLACK;
const BORDER_COLOUR: Color32 = Color32::from_rgb(100, 0, 100);
const BORDER_WIDTH: f32 = 2.0;

pub struct CustomButton<'a> {
    pub icon: TextureHandle,
    pub text: &'a str,
    pub on_click: Box<dyn FnMut() + 'a>,
    pub width: u32,
    pub height: u32,
    pub border_thickness: f32,
    pub border_colour: Color32,
    pub background_colour: Color32,
}


impl<'a> CustomButton<'a> {

    pub fn new(
        icon: TextureHandle,
        text: &'a str,
        on_click: Box<dyn FnMut() + 'a>,
    ) -> Self {
        Self {
            icon,
            text,
            on_click,
            width: BUTTON_SIZE.x as u32,
            height: BUTTON_SIZE.y as u32,
            border_thickness: BORDER_WIDTH,
            border_colour: BORDER_COLOUR,
            background_colour: BACKGROUND_COLOUR,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let button_size = Vec2::new(self.width as f32, self.height as f32);

        // Create an interactive area for the button
        let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());

        if response.clicked() {
            (self.on_click)();
        }

        // Draw button background
        ui.painter().rect_filled(
            rect,
            5.0, // Rounded corners
            self.background_colour,
        );

        // Draw border
        ui.painter().rect_stroke(rect, 5.0, Stroke::new(self.border_thickness, self.border_colour), StrokeKind::Inside);

        ui.allocate_ui_with_layout(Vec2::new(self.width as f32, self.height as f32), Layout::centered_and_justified(Direction::LeftToRight), |ui| {
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.add(Image::new(SizedTexture::new(self.icon.id(), Vec2::new(24.0, 24.0))));
                ui.add(Label::new(RichText::new(self.text).color(Color32::WHITE).strong()));
            });
        });

        response
    }
}