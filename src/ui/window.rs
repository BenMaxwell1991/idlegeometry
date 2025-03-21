use crate::enums::gametab::GameTab;
use crate::game::constants::{FRAME_RATE, GAME_NAME};
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, GAME_IN_FOCUS, SETTINGS};
use crate::game::settings::Settings;
use crate::ui::asset::loader::{load_icons, load_icons_inverted, register_custom_font};
use crate::ui::graphics::offscreen_renderer::OffscreenRenderer;
use crate::ui::panel::main_game::show_main_game;
use crate::ui::panel::settings::show_settings_panel;
use crate::ui::panel::shop::show_shop;
use crate::ui::panel::upgrades::show_upgrades;
use crate::ui::sidemenu::show_side_menu;
use eframe::egui::{Align, Color32, Context, Layout, Vec2};
use eframe::Renderer::Glow;
use eframe::{egui, App, Frame, NativeOptions};
use glow::HasContext;
use std::sync::Arc;
use std::thread::spawn;
use std::time::{Duration, Instant};

pub const BACKGROUND_COLOUR: Color32 = Color32::from_rgb(5, 5, 5);

pub struct MyAppWindow {
    game_data: Arc<GameData>,
    gl_ctx: Arc<glow::Context>,
    last_update: Instant,
}

impl MyAppWindow {
    pub fn new(game_data: Arc<GameData>, ctx: Context, gl: Arc<glow::Context>) -> Self {
        {
            let mut offscreen_renderer = game_data.offscreen_renderer.write().unwrap();
            let width = game_data.get_field(SETTINGS).unwrap().window_width;
            let height = game_data.get_field(SETTINGS).unwrap().window_height;
            *offscreen_renderer = Some(OffscreenRenderer::new(gl.clone(), width as i32, height as i32));
        }

        let frame_time = Duration::from_secs_f64(1.0 / FRAME_RATE);

        register_custom_font(&ctx);
        load_icons(&ctx, &game_data);
        load_icons_inverted(&ctx, &game_data);

        spawn(move || {
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
            gl_ctx: gl,
            last_update: Instant::now(),
        }
    }
}

impl App for MyAppWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.last_update = Instant::now();
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

        show_side_menu(ctx, &self.game_data);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                match current_tab {
                    GameTab::Adventure => show_main_game(ui, Arc::clone(&self.game_data), _frame),
                    GameTab::Settings => show_settings_panel(ui, &self.game_data),
                    GameTab::Shop => show_shop(ui, &self.game_data),
                    GameTab::Upgrades => show_upgrades(ui, &self.game_data),
                    GameTab::NullGameTab => (),
                }
            });
        });
    }
}

impl Drop for MyAppWindow {
    fn drop(&mut self) {
        if let Some(shader) = self.game_data.rect_shader.write().unwrap().take() {
            unsafe {
                self.gl_ctx.delete_program(shader);
                println!("✅ Deleted rectangle shader program when MyAppWindow was dropped.");
            }
        }
        if let Some(shader) = self.game_data.sprite_shader.write().unwrap().take() {
            unsafe {
                self.gl_ctx.delete_program(shader);
                println!("✅ Deleted sprite shader program when MyAppWindow was dropped.");
            }
        }
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
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([settings.window_width, settings.window_height])
            .with_title(GAME_NAME),
        vsync: settings.vsync,
        centered: true,
        renderer: Glow,
        ..Default::default()
    };

    eframe::run_native(
        GAME_NAME,
        options,
        Box::new(|cc| {
            if let Some(gl_ctx) = cc.gl.clone() {
                Ok(Box::new(MyAppWindow::new(Arc::clone(&game_data), cc.egui_ctx.clone(), gl_ctx)) as Box<dyn App>)
            } else {
                Err("Failed to get OpenGL context".into())
            }
        }),
    )
}
