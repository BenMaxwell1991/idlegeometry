use crate::game::data::resource_cost::ResourceAmount;
use crate::ui::asset::loader::{DP_COMIC_FONT, DRAGON_HEART_GEMSTONE_IMAGE, IMP_CHEF_IMAGE};
use crate::ui::component::widget::custom_progress_bar::CustomProgressBar;
use crate::ui::component::widget::label_no_interact::LabelNoInteract;
use eframe::egui::{
    Response, Sense, Ui
    , Widget,
};
use eframe::epaint::FontFamily;
use egui::{Align, Color32, FontId, Frame, Image, Layout, Stroke, TextureHandle, UiBuilder, Vec2};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use derivative::Derivative;

#[derive(Clone, Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct LairObject {
    pub name: String,
    pub level: u32,
    pub quantity: u32,
    pub icon_name: Option<String>,
    pub unlocked: bool,
    pub production_duration: u64,
    pub production_amount: ResourceAmount,
    pub production_cost: ResourceAmount,
    pub upgrade_cost: ResourceAmount,
    #[serde(skip)]
    pub size: Option<Vec2>,
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub icon: Option<TextureHandle>,
    #[serde(skip, default = "default_instant_now")]
    pub last_produced: Instant,
}

impl LairObject {
    pub fn new(name: impl Into<String>, level: u32, quantity: u32, size: Option<Vec2>, icon: Option<TextureHandle>, icon_name: Option<String>) -> Self {
        Self {
            name: name.into(),
            level,
            quantity,
            size,
            icon,
            icon_name,
            unlocked: false,
            production_duration: u64::MAX,
            upgrade_cost: ResourceAmount::default(),
            production_cost: ResourceAmount::default(),
            production_amount:  Default::default(),
            last_produced:  Instant::now(),
        }
    }
}

impl Widget for LairObject {
    fn ui(self, ui: &mut Ui) -> Response {
        let ui_size = self.size.unwrap_or(Vec2::new(300.0, 80.0));

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
                            // Icon on the left
                            if let Some(icon) = &self.icon {
                                ui.allocate_ui_with_layout(
                                    Vec2::new(rect.height(), rect.height()),
                                    Layout::top_down_justified(Align::Min),
                                    |ui| {
                                        ui.add(Image::new(icon).fit_to_exact_size(ui.available_size()));
                                    },
                                );
                            }

                            // Right side content
                            ui.vertical(|ui| {
                                Frame::group(ui.style()).stroke(Stroke::new(1.0, Color32::LIGHT_GREEN)).show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.add(LabelNoInteract::new(&self.name, DP_COMIC_FONT.to_string(), 20.0, Color32::WHITE));
                                        ui.add(LabelNoInteract::new(format!("Lvl {}", self.level).as_str(), DP_COMIC_FONT.to_string(), 20.0, Color32::WHITE));
                                    });
                                });

                                ui.horizontal(|ui| {
                                    Frame::group(ui.style()).stroke(Stroke::new(1.0, Color32::LIGHT_BLUE)).show(ui, |ui| {
                                        ui.add(LabelNoInteract::new("Left Panel Content", DP_COMIC_FONT.to_string(), 14.0, Color32::WHITE));
                                    });
                                    Frame::group(ui.style()).stroke(Stroke::new(1.0, Color32::LIGHT_RED)).show(ui, |ui| {
                                        ui.add(LabelNoInteract::new("Right Panel Content", DP_COMIC_FONT.to_string(), 14.0, Color32::WHITE));
                                    });
                                });

                                // Progress Bar for Production
                                let elapsed_ms = Instant::now().duration_since(self.last_produced).as_millis() as u64;
                                let progress = (elapsed_ms as f64).min(self.production_duration as f64);
                                let font_id = FontId::new(20.0, FontFamily::Name(DP_COMIC_FONT.into()));
                                ui.add(
                                    CustomProgressBar::new(progress, self.production_duration as f64)
                                        .show_percentage()
                                        .with_completed_text("Production Ready".to_string(), font_id)
                                        .set_on_click(Box::new(|| println!("Progress Bar Clicked")))
                                );
                            });
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
            quantity: 0,
            size: None,
            icon: None,
            icon_name: None,
            unlocked: false,
            production_duration: u64::MAX,
            production_amount: ResourceAmount::default(),
            production_cost: ResourceAmount::default(),
            upgrade_cost: ResourceAmount::default(),
            last_produced: Instant::now(),
        }
    }
}

pub fn get_lair_object(n: u32, level: u32) -> LairObject {
    match n {
        0 => { lair_object_00_heart(level, 1) }
        1 => { lair_object_01_imp_chef(level, 0) }
        _ => { LairObject::default() }
    }
}

pub fn lair_object_00_heart(level: u32, quantity: u32) -> LairObject {

    let multiplier_prod = 1 + level + level.pow(2) / 10;
    let production_amount = ResourceAmount {
        food: Some(1.0 * multiplier_prod as f64 * quantity as f64),
        ..Default::default()
    };

    let multiplier_cost = 1 + level + level.pow(2) / 100;
    let production_cost = ResourceAmount {
        gold: Some(5.0 * multiplier_cost as f64 * quantity as f64),
        ..Default::default()
    };

    LairObject {
        name: "Dragon's Heart".to_string(),
        level,
        quantity,
        size: None,
        icon: None,
        icon_name: Some(DRAGON_HEART_GEMSTONE_IMAGE.to_string()),
        unlocked: true,
        production_duration: 5_000,
        production_amount,
        production_cost,
        upgrade_cost: ResourceAmount::default(),
        last_produced: Instant::now(),
    }
}

pub fn lair_object_01_imp_chef(level: u32, quantity: u32) -> LairObject {
    let multiplier_prod =  level + level.pow(2) / 10;
    let production_amount = ResourceAmount {
        food: Some(1.0 * multiplier_prod as f64 * quantity as f64),
        ..Default::default()
    };

    let multiplier_cost = level + level.pow(2) / 100;
    let production_cost = ResourceAmount {
        gold: Some(5.0 * multiplier_cost as f64 * quantity as f64),
        ..Default::default()
    };

    LairObject {
        name: "Imp Chef".to_string(),
        level,
        quantity,
        size: None,
        icon: None,
        icon_name: Some(IMP_CHEF_IMAGE.to_string()),
        unlocked: false,
        production_duration: 50_000,
        production_amount,
        production_cost,
        upgrade_cost: ResourceAmount::default(),
        last_produced: Instant::now(),
    }
}


fn default_instant_now() -> Instant {
    Instant::now()
}