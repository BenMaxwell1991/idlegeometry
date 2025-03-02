use crate::ui::component::label_no_interact::LabelNoInteract;
use crate::ui::helper::colour_helper::lighten_colour;
use eframe::egui::load::SizedTexture;
use eframe::egui::{Align, Color32, Direction, FontId, Id, Image, Layout, Pos2, Response, Sense, Stroke, StrokeKind, TextureHandle, Ui, UiBuilder, Vec2};
use eframe::emath::Rect;

const BUTTON_SIZE: Vec2 = Vec2::new(200.0, 50.0);
const TEXT_COLOUR: Color32 = Color32::WHITE;
const BACKGROUND_COLOUR: Color32 = Color32::BLACK;
const BORDER_COLOUR: Color32 = Color32::from_rgb(100, 0, 100);
const BORDER_WIDTH: f32 = 2.0;
const ICON_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const GAP_SIZE: f32 = 0.0;
const FONT_SIZE: f32 = 24.0;

pub struct CustomButton<'a> {
    pub icon: TextureHandle,
    pub text: &'a str,
    pub on_click: Box<dyn FnMut() + 'a>,
    pub size: Vec2,
    pub border_thickness: f32,
    pub border_colour: Color32,
    pub background_colour: Color32,
    pub icon_size: Vec2,
    pub gap_size: f32,
    pub font_size: f32,
    pub hover_effect: bool,
    pub click_effect: bool,
    pub align: Align,
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
            size: BUTTON_SIZE,
            border_thickness: BORDER_WIDTH,
            border_colour: BORDER_COLOUR,
            background_colour: BACKGROUND_COLOUR,
            icon_size: ICON_SIZE,
            gap_size: GAP_SIZE,
            font_size: FONT_SIZE,
            hover_effect: true,
            click_effect: true,
            align: Align::Center,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let parent_rect = ui.available_rect_before_wrap();
        let mut rect = ui.allocate_exact_size(self.size, Sense::empty()).0;

        match self.align {
            Align::Center => rect = Rect::from_min_size(Pos2::new(parent_rect.center().x - self.size.x / 2.0, rect.min.y, ), self.size),
            _ => {}
        }

        let response = ui.interact(rect, ui.make_persistent_id(self.text), Sense::click());

        let button_clicked = self.button_clicked(ui, &response);
        let click_released_inside = self.released_inside(ui, &response);

        if click_released_inside {
            (self.on_click)();
        }

        let (background_colour, border_colour) = self.determine_colour(button_clicked, response.contains_pointer());

        ui.painter().rect_filled(
            rect,
            5.0,
            background_colour,
        );

        ui.painter().rect_stroke(rect, 5.0, Stroke::new(self.border_thickness, border_colour), StrokeKind::Inside);

        ui.allocate_new_ui(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
            |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(self.calculate_padding(&ui).max(0.0));
                    ui.add(Image::new(SizedTexture::new(self.icon.id(), self.icon_size)));
                    ui.add_space(self.gap_size);
                    ui.add(LabelNoInteract::new(self.text, self.font_size, TEXT_COLOUR));
                    ui.add_space(self.calculate_padding(&ui).max(0.0));
                });
            }
        );

        response
    }

    // Determines whether the button is currently clicked.
    fn button_clicked(&mut self, ui: &mut Ui, response: &Response) -> bool {
        let button_id = Id::new((self.text, self.icon.id()));
        let mut click_started_inside = ui.data(|data| data.get_temp::<bool>(button_id)).unwrap_or(false);
        let is_mouse_down = response.ctx.input(|i| i.pointer.primary_down());

        if is_mouse_down {
            let press_origin = response.ctx.input(|i| i.pointer.press_origin());
            click_started_inside = press_origin.map_or(false, |pos| response.rect.contains(pos));
            ui.data_mut(|data| data.insert_temp(button_id, click_started_inside));
        }

        is_mouse_down && click_started_inside
    }

    // Determines whether the mouse_up happens within the widget if it has been clicked.
    fn released_inside(&mut self, ui: &mut Ui, response: &Response) -> bool {
        let button_id = Id::new((self.text, self.icon.id()));
        let click_started_inside = ui.data(|data| data.get_temp::<bool>(button_id)).unwrap_or(false);
        let is_mouse_up = response.ctx.input(|i| i.pointer.primary_released());
        let release_inside = response.contains_pointer();

        is_mouse_up && click_started_inside && release_inside
    }

    // Determine colour of button, allowing for hovering/pressed effects
    fn determine_colour(&mut self, button_clicked: bool, point_hovering: bool) -> (Color32, Color32) {
        let mut background_colour = self.background_colour;
        let mut border_colour = self.border_colour;

        if self.click_effect && button_clicked {
            background_colour = lighten_colour(self.background_colour, -20);
            border_colour = lighten_colour(self.border_colour, -30);
        } else if self.hover_effect && point_hovering {
            background_colour = lighten_colour(self.background_colour, 25);
            border_colour = lighten_colour(self.border_colour, 40);
        }

        (background_colour, border_colour)
    }

    // Calculate padding for centered text
    fn calculate_padding(&mut self, ui: &Ui) -> f32 {
        let label_width = ui.fonts(|f| {
            f.layout_no_wrap(
                self.text.parse().unwrap(),
                FontId::proportional(self.font_size),
                Color32::WHITE,
            ).size().x
        });

        let desired_width = self.icon_size.x + label_width + self.gap_size;
        let available_width = ui.available_width();

        (available_width - desired_width) / 2.0
    }
}