use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::SETTINGS;
use crate::helper::lock_helper::acquire_lock;
use crate::ui::asset::loader::DP_COMIC_FONT;
use crate::ui::component::widget::custom_heading::CustomHeading;
use crate::ui::component::widget::custom_progress_bar::CustomProgressBar;
use crate::ui::component::widget::game_graphics::GameGraphics;
use crate::ui::panel::death_menu::show_death_menu;
use crate::ui::panel::game_menu_lair::show_begin_adventure;
use crate::ui::panel::game_menu_paused::show_game_menu_paused;
use eframe::{egui, Frame};
use egui::{scroll_area, Align, Color32, FontFamily, FontId, Image, Layout, Pos2, Rect, RichText, ScrollArea, Sense, StrokeKind, Ui, UiBuilder, Vec2};
use std::process::exit;
use std::sync::{Arc, OnceLock};
use egui::WidgetType::Label;
use uuid::Uuid;
use crate::ui::component::widget::label_no_interact::LabelNoInteract;
use crate::ui::component::widget::lair_object::LairObject;

static GAME_GRAPHICS_ID: OnceLock<Uuid> = OnceLock::new();
static RESOURCE_HUD_ID: OnceLock<Uuid> = OnceLock::new();

pub fn show_main_game(ui: &mut Ui, game_data: Arc<GameData>, frame: &mut Frame) {
    let game_state = acquire_lock(&game_data.game_state, "game_state").clone();

    match game_state {
        GameState::Lair => handle_game_state_lair(ui, &game_data),
        GameState::Playing => handle_game_state_playing(ui, &game_data, frame),
        GameState::Paused => handle_game_state_paused(ui, &game_data),
        GameState::Dead => handle_game_state_dead(ui, &game_data),
        GameState::Quitting => handle_game_state_quitting(),
    }
}

fn handle_game_state_lair(ui: &mut Ui, game_data: &GameData) {
    ui.add(CustomHeading::new("Dragons Lair"));
    ui.separator();
    let game_rect = ui.available_rect_before_wrap();

    draw_background_lair(ui, game_data, game_rect);
    draw_lair_objects(ui, game_data, game_rect);


    let hud_size = Vec2::new(210.0, 100.0);
    let hud_pos = Pos2::new(game_rect.min.x + 20.0, game_rect.min.y + 20.0);
    let hud_rect = Rect::from_min_size(hud_pos, hud_size);
    show_begin_adventure(ui, game_data, game_rect);
    draw_resource_hud_lair(ui, game_data, hud_rect);
}

fn handle_game_state_playing(ui: &mut Ui, game_data: &Arc<GameData>, frame: &mut Frame) {
    ui.add(CustomHeading::new("Adventure Mode"));
    ui.separator();
    let game_rect = ui.available_rect_before_wrap();

    ui.put(game_rect, GameGraphics::new(Arc::clone(game_data), frame));

    let (hud_rect, progress_rect) = get_hud_rects(&game_rect);

    draw_resource_hud_active(ui, game_data, hud_rect);

    if let Some(food_value) = acquire_lock(&game_data.resources, "resources").get("Food").cloned() {
        ui.put(
            progress_rect,
            CustomProgressBar::new(food_value, 100.0)
                .show_percentage()
                .set_on_click(Box::new(|| println!("Progress Bar Clicked"))),
        );
    }
}

fn handle_game_state_paused(ui: &mut Ui, game_data: &GameData) {
    ui.add(CustomHeading::new("Adventure Mode"));
    ui.separator();
    let game_rect = ui.available_rect_before_wrap();

    show_game_menu_paused(ui, game_data, game_rect);
}

fn handle_game_state_dead(ui: &mut Ui, game_data: &GameData) {
    ui.add(CustomHeading::new("Adventure Mode"));
    ui.separator();
    let game_rect = ui.available_rect_before_wrap();

    let icons = game_data.icons.read().unwrap();
    let dragon_picture = icons.get("dragon").cloned();

    if let Some(dragon) = &dragon_picture {
        let max = game_rect.width().max(game_rect.height());
        ui.add(
            Image::new(dragon)
                .fit_to_exact_size(Vec2::new(max, max))
                .tint(Color32::from_rgba_unmultiplied(96, 96, 96, 255)),
        );
    } else {
        ui.label("No death background loaded.");
    }

    show_death_menu(ui, game_data, game_rect);
}

fn handle_game_state_quitting() {
    exit(0);
}

fn draw_lair_objects(ui: &mut Ui, game_data: &GameData, game_rect: Rect) {
    let icons = game_data.icons.read().unwrap();
    let image = icons.get("dragons_heart").cloned();

    let widget_size = Vec2::new(500.0, 100.0);
    let spacing = 20.0;
    let num_objects = 50;

    let mut objects = Vec::new();
    let mut top = game_rect.top() + 20.0;

    for i in 0..num_objects {
        let center_x = game_rect.center().x;
        let rect = Rect::from_center_size(
            Pos2::new(center_x, top + widget_size.y / 2.0),
            widget_size,
    );

        objects.push(LairObject::new("Basic Worker", i + 1, rect, image.clone()));
        top += widget_size.y + spacing;
    }

    let scroll_area_width = widget_size.x;
    let scroll_area_height = game_rect.height() * 0.75;
    let scroll_origin_x = game_rect.center().x - scroll_area_width / 2.0;
    let scroll_rect = Rect::from_min_size(
        Pos2::new(scroll_origin_x, game_rect.top()),
        Vec2::new(scroll_area_width, scroll_area_height),
    );

    ui.allocate_new_ui(
        UiBuilder::new()
            .max_rect(scroll_rect)
            .layout(Layout::top_down(Align::Center)), |ui| {
            ScrollArea::vertical()
                .auto_shrink([true; 2])
                .max_height(scroll_area_height)
                .max_width(scroll_area_width)
                .show(ui, |ui| {
                for lair_object in objects.iter() {
                    ui.add(lair_object.clone());
                    ui.add_space(spacing);
                }
            })}
    );
}

fn draw_background_lair(ui: &mut Ui, game_data: &GameData, game_rect: Rect) {
    let icons = game_data.icons.read().unwrap();
    let dragons_lair_image = icons.get("dragons_lair").cloned();

    if let Some(image) = &dragons_lair_image {
        let max = game_rect.width().max(game_rect.height());
        ui.add(
            Image::new(image)
                .fit_to_exact_size(Vec2::new(max, max))
                .tint(Color32::from_rgba_unmultiplied(196, 196, 196, 255)),
        );
    } else {
        ui.label("No lair background loaded.");
    }
}

fn draw_resource_hud_lair(ui: &mut Ui, game_data: &GameData, hud_rect: Rect) {
    let settings = game_data.get_field(SETTINGS).unwrap();
    let resources = game_data.resources.read().unwrap();
    let icons = game_data.icons.read().unwrap();

    let gold = resources.get("Food").cloned().unwrap_or(0.0);
    let ruby = resources.get("Ruby").cloned().unwrap_or(0.0);

    let gold_icon = icons.get("coin").cloned();
    let ruby_icon = icons.get("ruby").cloned();

    let painter = ui.painter();

    painter.rect_filled(hud_rect, 10.0, Color32::from_rgb(65, 35, 10));
    painter.rect_stroke(hud_rect, 10.0, (2.0, Color32::from_rgb(128, 0, 128)), StrokeKind::Inside);

    ui.allocate_new_ui(
        UiBuilder::new()
            .max_rect(hud_rect)
            .layout(Layout::top_down_justified(Align::Min)),
        |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    if let Some(icon) = &gold_icon {
                        ui.add(Image::new(icon).fit_to_exact_size(Vec2::new(35.0, 35.0)));
                    }
                    ui.label(
                        RichText::new(format!("Gold: {:.0}", gold))
                            .font(FontId::new(42.0, FontFamily::Name(DP_COMIC_FONT.into())))
                            .color(Color32::GOLD)
                    );
                });

                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    if let Some(icon) = &ruby_icon {
                        ui.add(Image::new(icon).fit_to_exact_size(Vec2::new(35.0, 35.0)));
                    }
                    ui.label(
                        RichText::new(format!("Rubies: {:.0}", ruby))
                            .font(FontId::new(42.0, FontFamily::Name(DP_COMIC_FONT.into())))
                            .color(Color32::from_rgb(255, 50, 50)),
                    );
                });
            });
        },
    );
}
fn draw_resource_hud_active(ui: &mut Ui, game_data: &GameData, hud_rect: Rect) {
    let settings = game_data.get_field(SETTINGS).unwrap();
    let resources = game_data.resources.read().unwrap();
    let icons = game_data.icons.read().unwrap();

    let gold = resources.get("Gold").cloned().unwrap_or(0.0);
    let ruby = resources.get("Ruby").cloned().unwrap_or(0.0);

    let gold_icon = icons.get("coin").cloned();
    let ruby_icon = icons.get("ruby").cloned();

    let painter = ui.painter();

    painter.rect_filled(hud_rect, 10.0, Color32::from_rgb(65, 35, 10));
    painter.rect_stroke(hud_rect, 10.0, (2.0, Color32::from_rgb(128, 0, 128)), StrokeKind::Inside);

    ui.allocate_new_ui(
        UiBuilder::new()
            .max_rect(hud_rect)
            .layout(Layout::top_down_justified(Align::Min)),
        |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    if let Some(icon) = &gold_icon {
                        ui.add(Image::new(icon).fit_to_exact_size(Vec2::new(35.0, 35.0)));
                    }
                    ui.label(
                        RichText::new(format!("Gold: {:.0}", gold))
                            .font(FontId::new(42.0, FontFamily::Name(DP_COMIC_FONT.into())))
                            .color(Color32::GOLD)
                    );
                });

                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    if let Some(icon) = &ruby_icon {
                        ui.add(Image::new(icon).fit_to_exact_size(Vec2::new(35.0, 35.0)));
                    }
                    ui.label(
                        RichText::new(format!("Rubies: {:.0}", ruby))
                            .font(FontId::new(42.0, FontFamily::Name(DP_COMIC_FONT.into())))
                            .color(Color32::from_rgb(255, 50, 50)),
                    );
                });
            });
        },
    );
}

fn get_hud_rects(game_rect: &Rect) -> (Rect, Rect) {
    let hud_size = Vec2::new(210.0, 100.0);
    let hud_pos = Pos2::new(game_rect.min.x + 20.0, game_rect.min.y + 20.0);
    let hud_rect = Rect::from_min_size(hud_pos, hud_size);

    let progress_size = Vec2::new(400.0, 30.0);
    let progress_pos = Pos2::new(
        game_rect.center().x - progress_size.x / 2.0,
        game_rect.max.y - 60.0
    );
    let progress_rect = Rect::from_min_size(progress_pos, progress_size);

    (hud_rect, progress_rect)
}