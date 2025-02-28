use eframe::egui::load::SizedTexture;
use eframe::egui::{self, Align, Color32, Direction, Image, Layout, Response, RichText, TextureHandle, Ui, Vec2, WidgetText};

const BUTTON_SIZE: [f32; 2] = [130.0, 40.0];
const IMAGE_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const TEXT_COLOUR: Color32 = Color32::WHITE;
const BACKGROUND_COLOUR: Color32 = Color32::BLACK;
const BORDER_COLOUR: Color32 = Color32::from_rgb(100, 0, 100);
const BORDER_WIDTH: f32 = 2.0;

pub struct MenuButton<'a> {
    pub icon: TextureHandle,
    pub text: &'a str,
    pub on_click: Box<dyn FnMut() + 'a>,
}

impl<'a> MenuButton<'a> {
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let button_response = ui.add_sized(
            BUTTON_SIZE,
            egui::Button::image_and_text(
                Image::new(SizedTexture::new(self.icon.id(), IMAGE_SIZE)),
                WidgetText::RichText(RichText::new(self.text).color(TEXT_COLOUR).strong())
            )
            .fill(BACKGROUND_COLOUR)
            .stroke(egui::Stroke::new(BORDER_WIDTH, BORDER_COLOUR))
        );

        if button_response.clicked() {
            (self.on_click)();
        }

        button_response
    }
}
