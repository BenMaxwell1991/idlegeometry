use eframe::egui;
use eframe::egui::{InnerResponse, Layout, Pos2, Rect, Ui, UiBuilder, Vec2};

pub fn centered_ui<R>(
    ui: &mut Ui,
    widget_width: f32,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let available_width = ui.available_width();
    let x_offset = (available_width - widget_width).max(0.0) / 2.0;

    let rect = ui.max_rect();
    let widget_rect = Rect::from_min_size(
        Pos2::new(rect.min.x + x_offset, rect.min.y),
        Vec2::new(widget_width, rect.height()),
    );

    ui.allocate_new_ui(
        UiBuilder::new()
            .max_rect(widget_rect)
            .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
        add_contents,
    )
}
