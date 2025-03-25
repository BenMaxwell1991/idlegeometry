use crate::ui::component::widget::interactive_widget::InteractiveWidget;
use eframe::egui::{Align2, Color32, Pos2, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget};
use eframe::emath::{Align, Rect};
use eframe::epaint::FontId;
use egui::{Direction, Layout, RichText, UiBuilder};
use crate::ui::asset::loader::DP_COMIC_FONT;
use crate::ui::component::widget::label_no_interact::LabelNoInteract;

const TEXT_COLOUR: Color32 = Color32::WHITE;
const BORDER_COLOUR: Color32 = Color32::from_rgb(100, 0, 100);
const BORDER_WIDTH: f32 = 2.0;
const BACKGROUND_COLOUR: Color32 = Color32::DARK_GRAY;

pub struct CustomProgressBar<'a> {
    resource_current: f64,
    resource_max: f64,
    show_percentage: bool,
    on_click: Box<dyn FnMut() + 'a>,
    border_thickness: f32,
    border_colour: Color32,
    background_colour: Color32,
    draw_completed: bool,
    completed_text: Option<String>,
    completed_font_id: Option<FontId>,
}

impl<'a> CustomProgressBar<'a> {
    pub fn new(resource_current: f64, resource_max: f64) -> Self {
        Self {
            resource_current,
            resource_max,
            show_percentage: false,
            on_click: Box::new(|| {}),
            border_thickness: BORDER_WIDTH,
            border_colour: BORDER_COLOUR,
            background_colour: BACKGROUND_COLOUR,
            draw_completed: true,
            completed_text: None,
            completed_font_id: None,
        }
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub fn set_on_click(mut self, on_click: Box<dyn FnMut() + 'a>) -> Self {
        self.on_click = on_click;
        self
    }

    pub fn with_completed_text(mut self, text_str: String, font_id: FontId) -> Self {
        self.draw_completed = true;
        self.completed_text = Some(text_str.clone());
        self.completed_font_id = Some(font_id.clone());
        self
    }

    fn draw_completed(&self, ui: &mut Ui, response: &Response) {
        let painter = ui.painter();
        let rect = response.rect;
        let text_pos = Pos2::new(rect.center().x, rect.center().y);

        if let Some(font_id) = self.completed_font_id.clone() {
            if let Some(font_text) = &self.completed_text {
                let label_size = if let Some(text) = &self.completed_text {
                    ui.fonts(|f| {
                        f.layout_no_wrap(text.to_string(), font_id.clone(), TEXT_COLOUR)
                            .size()
                    })
                } else {
                    Vec2::default()
                };

                let label_rect = Rect::from_center_size(text_pos, label_size);

                ui.allocate_new_ui(
                    UiBuilder::new()
                        .max_rect(label_rect)
                        .layout(Layout::centered_and_justified(Direction::TopDown)),
                    |ui| {
                        ui.add(LabelNoInteract::new(font_text, DP_COMIC_FONT.to_string(), 25.0, Color32::PURPLE));
                    });
            }
        } else {
            painter.text(
                text_pos,
                Align2::CENTER_CENTER,
                "Level Up!",
                FontId::proportional(14.0),
                Color32::PURPLE,
            );
        }
    }
}

impl<'a> InteractiveWidget for CustomProgressBar<'a> {
    fn click_colours(&self) -> Vec<i8> {
        vec![-40, -25, -30]
    }

    fn hover_colours(&self) -> Vec<i8> {
        vec![30, 30, 35]
    }
}

impl<'a> Widget for CustomProgressBar<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let progress = &self.resource_current / &self.resource_max;
        let progress_completed = progress >= 1.0;
        let size = Vec2::new(ui.available_width(), ui.available_height());

        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        let bar_colour = if progress_completed { Color32::from_rgb(0, 200, 0) } else { Color32::from_rgb(200, 0, 0) };

        let button_clicked = self.button_clicked(ui, &response);
        let click_released_inside = self.released_inside(ui, &response);
        let base_colours = vec!(bar_colour, self.background_colour, self.border_colour);
        let [bar_colour, background_colour, border_colour]: [Color32; 3] =
            self.determine_colour(ui, base_colours, button_clicked, response.contains_pointer())
                .try_into()
                .expect("Invalid Array Size");

        if click_released_inside {
            (self.on_click)();
        }

        let painter = ui.painter();

        painter.rect_filled(rect, 4.0, background_colour);
        painter.rect_stroke(rect, 4.0, Stroke::new(self.border_thickness, border_colour), StrokeKind::Inside);

        let inner_rect = rect.shrink(self.border_thickness);
        let filled_width = inner_rect.width() * progress.min(1.0) as f32;
        let filled_rect = Rect {
            min: inner_rect.min,
            max: Pos2::new(inner_rect.min.x + filled_width, inner_rect.max.y),
        };
        painter.rect_filled(filled_rect, 4.0, bar_colour);

        if progress_completed && self.draw_completed {
            self.draw_completed(ui, &response);
        } else if self.show_percentage {
            draw_percentage(ui, progress.min(1.0), &response);
        }

        response
    }
}

fn draw_percentage(ui: &Ui, progress: f64, response: &Response) {
    let percentage_text = format!("{:.1}%", progress * 100.0);
    let painter = ui.painter();
    let rect = response.rect;

    let text_pos = Pos2::new(rect.center().x, rect.center().y);

    painter.text(
        text_pos,
        Align2::CENTER_CENTER,
        percentage_text,
        egui::FontId::proportional(14.0),
        TEXT_COLOUR,
    );
}
