use crate::game::data::resource_cost::ResourceAmount;
use crate::ui::asset::loader::{DP_COMIC_FONT, DRAGON_HEART_GEMSTONE_IMAGE, IMP_CHEF_IMAGE};
use crate::ui::component::widget::label_no_interact::LabelNoInteract;
use eframe::egui::{
    Response, Sense, Ui
    , Widget,
};
use egui::{Align, Color32, Frame, Image, Layout, Stroke, TextureHandle, UiBuilder, Vec2};

#[derive(Clone)]
pub struct LairObject {
    pub name: String,
    pub level: u32,
    pub size: Option<Vec2>,
    pub icon: Option<TextureHandle>,
    pub icon_name: Option<String>,
    pub unlocked: bool,
    pub production_duration: u64,
    pub production_amount: ResourceAmount,
    pub production_cost: ResourceAmount,
    pub upgrade_cost: ResourceAmount,
}

impl LairObject {
    pub fn new(name: impl Into<String>, level: u32, size: Option<Vec2>, icon: Option<TextureHandle>, icon_name: Option<String>) -> Self {
        Self {
            name: name.into(),
            level,
            size,
            icon,
            icon_name,
            unlocked: false,
            production_duration: u64::MAX,
            upgrade_cost: ResourceAmount::default(),
            production_cost: ResourceAmount::default(),
            production_amount:  Default::default(),
        }
    }
}


impl Widget for LairObject {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut ui_size = Vec2::new(0.0,0.0);
        if let Some(size) = self.size {
            ui_size = size;
        }

        let (rect, response) = ui.allocate_exact_size(ui_size, Sense::click());

        ui.allocate_new_ui(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down_justified(Align::Center)),
            |ui| {
                Frame::group(ui.style())
                    .stroke(Stroke::new(2.0, Color32::PURPLE))
                    .fill(Color32::from_rgba_premultiplied(0, 0, 0, 100))
                    .show(ui, |ui| {
                        ui.set_min_size(rect.size());
                        ui.horizontal(|ui| {
                            ui.allocate_ui_with_layout(
                                Vec2::new(rect.height(), rect.height()),
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
                                Vec2::new(ui.available_width(), rect.height()),
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

impl Default for LairObject {
    fn default() -> Self {
        Self {
            name: "Empty".to_string(),
            level: 0,
            size: None,
            icon: None,
            icon_name: None,
            unlocked: false,
            production_duration: u64::MAX,
            production_amount: ResourceAmount::default(),
            production_cost: ResourceAmount::default(),
            upgrade_cost: ResourceAmount::default(),
        }
    }
}

pub fn get_lair_object(n: u32, level: u32) -> LairObject {
    match n {
        0 => { lair_object_00_heart(level) }
        1 => { lair_object_01_imp_chef(level) }
        _ => { LairObject::default() }
    }
}

pub fn lair_object_00_heart(level: u32) -> LairObject {

    let multiplier_prod = level + level.pow(2) / 10;
    let production_amount = ResourceAmount {
        food: Some(1.0 * multiplier_prod as f64),
        ..Default::default()
    };

    LairObject {
        name: "Dragon's Heart".to_string(),
        level,
        size: None,
        icon: None,
        icon_name: Some(DRAGON_HEART_GEMSTONE_IMAGE.to_string()),
        unlocked: true,
        production_duration: 5_000,
        production_amount,
        production_cost: ResourceAmount::default(),
        upgrade_cost: ResourceAmount::default(),
    }
}

pub fn lair_object_01_imp_chef(level: u32) -> LairObject {
    let multiplier_prod =  level + level.pow(2) / 10;
    let production_amount = ResourceAmount {
        food: Some(1.0 * multiplier_prod as f64),
        ..Default::default()
    };

    let multiplier_cost = level + level.pow(2) / 100;
    let production_cost = ResourceAmount {
        gold: Some(5.0 * multiplier_cost as f64),
        ..Default::default()
    };

    LairObject {
        name: "Imp Chef".to_string(),
        level,
        size: None,
        icon: None,
        icon_name: Some(IMP_CHEF_IMAGE.to_string()),
        unlocked: false,
        production_duration: 50_000,
        production_amount,
        production_cost,
        upgrade_cost: ResourceAmount::default(),
    }
}