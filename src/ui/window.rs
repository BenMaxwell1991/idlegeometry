use crate::enums::gametab::GameTab;
use crate::game::constants::{FRAME_RATE, GAME_NAME};
use crate::game::game::Game;
use crate::ui::asset::loader::{load_icons, load_icons_inverted};
use crate::ui::geometry::show_geometry;
use crate::ui::settingspanel::show_settings_panel;
use crate::ui::sidemenu::show_side_menu;
use crossbeam::channel::Receiver;
use eframe::egui::{Context, TextureHandle};
use eframe::{egui, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct MyAppWindow {
    game: Arc<Mutex<Game>>,
    receiver: Receiver<()>,
    icons: HashMap<String, TextureHandle>,
    icons_inverted: HashMap<String, TextureHandle>,
}

impl MyAppWindow {
    pub fn new(game: Arc<Mutex<Game>>, receiver: Receiver<()>, ctx: egui::Context) -> Self {
        let frame_time = Duration::from_secs_f64(1.0/FRAME_RATE);

        let icons = load_icons(&ctx);
        let icons_inverted = load_icons_inverted(&ctx);

        thread::spawn(move || {
            let mut last_frame = Instant::now();
            loop {
                let now = Instant::now();
                let elapsed = now.duration_since(last_frame);
                if elapsed >= frame_time {
                    ctx.request_repaint();
                    last_frame = now;
                }
                thread::sleep(frame_time.saturating_sub(elapsed));
            }
        });

        Self { game, receiver, icons, icons_inverted }
    }
}

impl eframe::App for MyAppWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        while self.receiver.try_recv().is_ok() {}

        show_side_menu(ctx, Arc::clone(&self.game), &self.icons_inverted);

        let mut game_tab = GameTab::Geometry;
        if let Ok(game) = &self.game.lock() {
            game_tab = game.current_tab;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match game_tab {
                GameTab::Geometry => show_geometry(ui, Arc::clone(&self.game)),
                GameTab::Settings => show_settings_panel(ui, Arc::clone(&self.game)),
                _ => {}
            }
        });
    }
}

pub fn create_window(game: Arc<Mutex<Game>>, receiver: Receiver<()>) -> eframe::Result {
    let settings = game.lock().unwrap().settings;
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
        Box::new(|cc| Ok(Box::new(MyAppWindow::new(game, receiver, cc.egui_ctx.clone())) as Box<dyn eframe::App>)),
    )
}