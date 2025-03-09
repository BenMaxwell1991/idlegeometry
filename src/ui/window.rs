use crate::enums::gametab::GameTab;
use crate::game::constants::{FRAME_RATE, GAME_NAME};
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, GAME_IN_FOCUS, SETTINGS};
use crate::game::settings::Settings;
use crate::ui::asset::loader::{load_icons, load_icons_inverted, load_sprite_sheets};
use crate::ui::panel::main_game::show_main_game;
use crate::ui::panel::settings::show_settings_panel;
use crate::ui::panel::shop::show_shop;
use crate::ui::panel::upgrades::show_upgrades;
use crate::ui::sidemenu::show_side_menu;
use eframe::egui::{Align, Color32, Context, Layout, TextureHandle, Vec2};
use eframe::{egui, Frame};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

pub const BACKGROUND_COLOUR: Color32 = Color32::from_rgb(5, 5, 5);

pub struct MyAppWindow {
    game_data: Arc<GameData>,
    icons: HashMap<String, TextureHandle>,
    icons_inverted: HashMap<String, TextureHandle>,
}

impl MyAppWindow {
    pub fn new(game_data: Arc<GameData>, ctx: Context) -> Self {
        let frame_time = Duration::from_secs_f64(1.0 / FRAME_RATE);

        let icons = load_icons(&ctx);
        let icons_inverted = load_icons_inverted(&ctx);
        load_sprite_sheets(&ctx, &game_data);

        thread::spawn(move || {
            let mut last_frame = Instant::now();
            loop {
                let now = Instant::now();
                let elapsed = now.duration_since(last_frame);
                if elapsed >= frame_time {
                    ctx.request_repaint();
                    last_frame = now;
                }
            }
        });

        Self {
            game_data,
            icons,
            icons_inverted,
        }
    }
}

impl eframe::App for MyAppWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.game_data.update_or_set(GAME_IN_FOCUS, false, |in_focus| { *in_focus = ctx.input(|i| i.focused) });

        let settings = self.game_data.get_field(SETTINGS).unwrap_or_default();
        let current_tab = self
            .game_data
            .get_field(CURRENT_TAB)
            .unwrap_or(GameTab::default());

        set_window_size(ctx, &settings);

        ctx.set_visuals(egui::Visuals {
            panel_fill: BACKGROUND_COLOUR,
            ..egui::Visuals::dark()
        });

        show_side_menu(ctx, Arc::clone(&self.game_data), &self.icons_inverted);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                match current_tab {
                    GameTab::Adventure => show_main_game(ui, Arc::clone(&self.game_data)),
                    GameTab::Settings => show_settings_panel(ui, Arc::clone(&self.game_data)),
                    GameTab::Shop => show_shop(ui, Arc::clone(&self.game_data), &self.icons_inverted),
                    GameTab::Upgrades => show_upgrades(ui, Arc::clone(&self.game_data)),
                    GameTab::NullGameTab => (),
                }
            });
        });
    }
}

fn set_window_size(ctx: &Context, settings: &Settings) {
    static mut LAST_WINDOW_SIZE: (f32, f32) = (0.0, 0.0);

    let current_size = (settings.window_width, settings.window_height);
    unsafe {
        if LAST_WINDOW_SIZE != current_size {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::from([
                settings.window_width,
                settings.window_height,
            ])));
            LAST_WINDOW_SIZE = current_size;
        }
    }
}

pub fn create_window(game_data: Arc<GameData>) -> eframe::Result {
    let settings = game_data.get_field(SETTINGS).unwrap_or_default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([settings.window_width, settings.window_height])
            .with_title(GAME_NAME),
        vsync: settings.vsync,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        GAME_NAME,
        options,
        Box::new(|cc| {
            Ok(Box::new(MyAppWindow::new(
                game_data,
                cc.egui_ctx.clone(),
            )) as Box<dyn eframe::App>)
        }),
    )
}
