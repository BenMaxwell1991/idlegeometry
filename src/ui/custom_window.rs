use std::sync::{Arc};
use crossbeam::channel::Receiver;
use eframe::{App, Frame};
use egui::{Context, ViewportId};
use egui_winit::State;
use winit::{
    application::{ApplicationHandler},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
use crate::game::game_data::GameData;
use crate::ui::window::MyAppWindow;

pub struct CustomWindow {
    window: Option<Window>,
    egui_ctx: Context,
    egui_state: Option<State>,
    app: Option<MyAppWindow>, // Holds the MyAppWindow instance
    game_data: Arc<GameData>,
    receiver: Receiver<()>,
    frame: Frame,
}

impl CustomWindow {
    pub fn new(game_data: Arc<GameData>, receiver: Receiver<()>, frame: Frame) -> Self {
        Self {
            window: None,
            egui_ctx: Context::default(),
            egui_state: None,
            app: None,
            game_data,
            receiver,
            frame,
        }
    }
}

impl ApplicationHandler for CustomWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();

        // Initialize egui_winit state
        let egui_state = State::new(
            self.egui_ctx.clone(),
            ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            window.theme(),
            None,
        );

        // Initialize the MyAppWindow instance
        let app = MyAppWindow::new(self.game_data.clone(), self.receiver.clone(), self.egui_ctx.clone());

        self.egui_state = Some(egui_state);
        self.app = Some(app);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(ref mut egui_state) = self.egui_state {
            if egui_state.on_window_event(self.window.as_ref().unwrap(), &event).consumed {
                return;
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    let raw_input = self.egui_state.as_mut().unwrap().take_egui_input(window);
                    let full_output = self.egui_ctx.run(raw_input, |ctx| {
                        if let Some(ref mut app) = self.app {
                            app.update(ctx, &mut self.frame);
                        }
                    });

                    // Handle platform-specific output
                    self.egui_state
                        .as_mut()
                        .unwrap()
                        .handle_platform_output(window, full_output.platform_output);

                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
