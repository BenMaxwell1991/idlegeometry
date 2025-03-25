use crate::game::data::experience_data::ExperienceData;
use crate::game::data::resource_cost::ResourceAmount;
use crate::ui::asset::loader::{DP_COMIC_FONT, DRAGON_HEART_GEMSTONE_IMAGE, IMP_CHEF_IMAGE};
use crate::ui::component::widget::custom_progress_bar::CustomProgressBar;
use crate::ui::component::widget::label_no_interact::LabelNoInteract;
use derivative::Derivative;
use eframe::egui::{
    Response, Sense, Ui
    , Widget,
};
use eframe::epaint::FontFamily;
use egui::{Align, Color32, Direction, FontId, Frame, Image, Layout, Margin, Stroke, TextureHandle, UiBuilder, Vec2};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::ui::component::widget::custom_button::CustomButton;

#[derive(Clone, Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct LairObject {
    pub name: String,
    pub experience_data: ExperienceData,
    pub quantity: u32,
    pub icon_name: Option<String>,
    pub active: bool,
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
    pub fn new(name: impl Into<String>, experience_data: ExperienceData, quantity: u32, size: Option<Vec2>, icon: Option<TextureHandle>, icon_name: Option<String>) -> Self {
        Self {
            name: name.into(),
            experience_data,
            quantity,
            size,
            icon,
            icon_name,
            active: false,
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
                ui.scope(|ui| {
                    if !self.unlocked {
                        ui.disable();
                    }
                    Frame::group(ui.style())
                        .stroke(Stroke::new(2.0, Color32::PURPLE))
                        .fill(Color32::from_rgba_premultiplied(0, 0, 0, 100))
                        .show(ui, |ui| {
                            ui.set_min_size(rect.size());

                            ui.horizontal(|ui| {
                                // Lair Object Icon
                                if let Some(icon) = &self.icon {
                                    ui.allocate_ui_with_layout(
                                        Vec2::new(rect.height(), rect.height()),
                                        Layout::top_down_justified(Align::Min),
                                        |ui| {
                                            ui.add(Image::new(icon).fit_to_exact_size(ui.available_size()));
                                        },
                                    );
                                }

                                ui.scope(|ui| {
                                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                                    ui.vertical(|ui| {
                                        let remaining_size = ui.available_size();
                                        let widget_width = remaining_size.x - 150.0;
                                        let widget_height = remaining_size.y / 3.5;
                                        let widget_spacing = remaining_size.y / 20.0;

                                        // Title Bar
                                        Frame::group(ui.style())
                                            .stroke(Stroke::new(1.0, Color32::LIGHT_GREEN))
                                            .inner_margin(Margin::same(0))
                                            .outer_margin(Margin::same(0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    let text = format!("{}   Lvl {}", self.name, self.experience_data.level);
                                                    ui.add_sized([widget_width, widget_height], LabelNoInteract::new(&text, DP_COMIC_FONT.to_string(), 20.0, Color32::WHITE));
                                                });
                                            });

                                        ui.add_space(widget_spacing);

                                        // Experience bar
                                        let exp_for_level_up = 100.0 + 10.0 * self.experience_data.level as f64;
                                        let progress = (self.experience_data.experience).min(exp_for_level_up);
                                        let font_id = FontId::new(20.0, FontFamily::Name(DP_COMIC_FONT.into()));
                                        ui.add_sized(
                                            [widget_width, widget_height],
                                            CustomProgressBar::new(progress, exp_for_level_up)
                                                .show_percentage(true)
                                                .with_completed_text("Level Up".to_string(), font_id)
                                                .set_on_click(Box::new(|| println!("Experience Bar Clicked")))
                                        );

                                        ui.add_space(widget_spacing);

                                        // Progress Bar for Production
                                        let elapsed_ms = Instant::now().duration_since(self.last_produced).as_millis() as u64;
                                        let progress = (elapsed_ms as f64).min(self.production_duration as f64);
                                        let font_id = FontId::new(20.0, FontFamily::Name(DP_COMIC_FONT.into()));
                                        ui.add_sized(
                                            [widget_width, widget_height],
                                            CustomProgressBar::new(progress, self.production_duration as f64)
                                                .show_percentage(self.active)
                                                .with_completed_text("Production Ready".to_string(), font_id)
                                                .set_on_click(Box::new(|| println!("Progress Bar Clicked")))
                                        );
                                    });
                                });

                                // Upgrade Buttons:
                                ui.scope(|ui| {
                                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                                    ui.vertical(|ui| {
                                        let remaining_size = ui.available_size();
                                        let widget_width = remaining_size.x;
                                        let widget_height = remaining_size.y / 2.0;

                                        let button_size = Vec2::new(widget_width, widget_height);
                                        let button_id = format!("{}-purchase-button", self.name);

                                        ui.add_sized(
                                            [widget_width, widget_height],
                                            CustomButton::new(self.icon, Some("Purchase"), Box::new(move || {}))
                                                .with_size(button_size)
                                                .with_font(DP_COMIC_FONT.to_string(), 10.0)
                                                .with_id(button_id)
                                        )
                                    });
                                });
                            });
                        });
                });
                if !self.active {
                    let full_rect = ui.min_rect();
                    let painter = ui.painter_at(full_rect);
                    painter.rect_filled(full_rect, 2.0, Color32::from_rgba_premultiplied(0, 0, 0, 0));
                }
            },
        );
        response
    }
}

impl Default for LairObject {
    fn default() -> Self {
        Self {
            name: "Empty".to_string(),
            experience_data: ExperienceData::default(),
            quantity: 0,
            size: None,
            icon: None,
            icon_name: None,
            active: false,
            unlocked: false,
            production_duration: u64::MAX,
            production_amount: ResourceAmount::default(),
            production_cost: ResourceAmount::default(),
            upgrade_cost: ResourceAmount::default(),
            last_produced: Instant::now(),
        }
    }
}

pub fn get_lair_object(n: u32, experience_data: ExperienceData) -> LairObject {
    match n {
        0 => { lair_object_00_heart(experience_data, 1) }
        1 => { lair_object_01_imp_chef(experience_data, 0) }
        _ => { LairObject::default() }
    }
}

pub fn lair_object_00_heart(experience_data: ExperienceData, quantity: u32) -> LairObject {

    let level = experience_data.level;

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
        experience_data,
        quantity,
        size: None,
        icon: None,
        icon_name: Some(DRAGON_HEART_GEMSTONE_IMAGE.to_string()),
        active: true,
        unlocked: true,
        production_duration: 5_000,
        production_amount,
        production_cost,
        upgrade_cost: ResourceAmount::default(),
        last_produced: Instant::now(),
    }
}

pub fn lair_object_01_imp_chef(experience_data: ExperienceData, quantity: u32) -> LairObject {
    let level = experience_data.level;

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
        experience_data,
        quantity,
        size: None,
        icon: None,
        icon_name: Some(IMP_CHEF_IMAGE.to_string()),
        active: false,
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