use crate::ui::asset::loader::DP_COMIC_FONT;
use crate::ui::component::widget::label_no_interact::LabelNoInteract;
use eframe::egui::{
    Rect, Response, Sense, Ui
    , Widget,
};
use egui::{Align, Color32, Frame, Image, Layout, Stroke, TextureHandle, UiBuilder, Vec2};
use uuid::Uuid;
use crate::game::data::resource_cost::ResourceAmount;

pub struct LairObject {
    pub name: String,
    pub level: u64,
    pub rect: Rect,
    pub icon: Option<TextureHandle>,
    pub unlocked: bool,
    pub upgrade_cost: ResourceAmount,
    pub production_cost: ResourceAmount,
}

impl LairObject {
    pub fn new(name: impl Into<String>, level: u64, rect: Rect, icon: Option<TextureHandle>) -> Self {
        Self {
            name: name.into(),
            level,
            rect,
            icon,
            unlocked: false,
            upgrade_cost: ResourceAmount::default(),
            production_cost: ResourceAmount::default(),
        }
    }
}


impl Widget for LairObject {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = ui.make_persistent_id(Uuid::new_v4());
        let response = ui.interact(self.rect, id, Sense::click());

        ui.allocate_new_ui(
            UiBuilder::new()
                .max_rect(self.rect)
                .layout(Layout::top_down_justified(Align::Center)),
            |ui| {
                Frame::group(ui.style())
                    .stroke(Stroke::new(2.0, Color32::PURPLE))
                    .fill(Color32::from_rgba_premultiplied(0, 0, 0, 100))
                    .show(ui, |ui| {
                        ui.set_min_size(self.rect.size());
                        ui.horizontal(|ui| {
                            ui.allocate_ui_with_layout(
                                Vec2::new(self.rect.height(), self.rect.height()),
                                Layout::top_down_justified(Align::Min),
                                |ui| {
                                    if let Some(icon) = &self.icon {
                                        ui.add(
                                            Image::new(icon)
                                                .fit_to_exact_size(ui.available_size())
                                                .tint(Color32::from_rgba_unmultiplied(196, 196, 196, 255)),
                                        );
                                    }
                                },
                            );

                            // Right: remaining space
                            ui.allocate_ui_with_layout(
                                Vec2::new(ui.available_width(), self.rect.height()),
                                Layout::top_down(Align::Min),
                                |ui| {
                                    ui.vertical(|ui| {
                                        Frame::group(ui.style())
                                            .stroke(Stroke::new(1.0, Color32::LIGHT_GREEN))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add(LabelNoInteract::new(
                                                        &self.name,
                                                        DP_COMIC_FONT.to_string(),
                                                        20.0,
                                                        Color32::WHITE,
                                                    ));
                                                    ui.add(LabelNoInteract::new(
                                                        format!("Lvl {}", self.level).as_str(),
                                                        DP_COMIC_FONT.to_string(),
                                                        20.0,
                                                        Color32::WHITE,
                                                    ));
                                                });
                                            });

                                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                            Frame::group(ui.style())
                                                .stroke(Stroke::new(1.0, Color32::LIGHT_BLUE))
                                                .show(ui, |ui| {
                                                    ui.add(LabelNoInteract::new(
                                                        "Left Panel Content",
                                                        DP_COMIC_FONT.to_string(),
                                                        14.0,
                                                        Color32::WHITE,
                                                    ));
                                                    ui.add(LabelNoInteract::new(
                                                        "+2 Gold/sec",
                                                        DP_COMIC_FONT.to_string(),
                                                        14.0,
                                                        Color32::CYAN,
                                                    ));
                                                });

                                            Frame::group(ui.style())
                                                .stroke(Stroke::new(1.0, Color32::LIGHT_RED))
                                                .show(ui, |ui| {
                                                    ui.add(LabelNoInteract::new(
                                                        "Right Panel Content",
                                                        DP_COMIC_FONT.to_string(),
                                                        14.0,
                                                        Color32::WHITE,
                                                    ));
                                                    ui.add(LabelNoInteract::new(
                                                        "-1 Ore/sec",
                                                        DP_COMIC_FONT.to_string(),
                                                        14.0,
                                                        Color32::MAGENTA,
                                                    ));
                                                });
                                        });
                                    });
                                },
                            );
                        });
                    });
            },
        );
        response
    }
}