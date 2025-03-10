use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CAMERA_STATE, KEY_STATE};
use device_query_revamped::{DeviceQuery, DeviceState, Keycode};
use rdev::{listen, EventType};
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct InputListener {
    game_data: Arc<GameData>,
    target_zoom: Arc<Mutex<f32>>,
}

impl InputListener {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self {
            game_data,
            target_zoom: Arc::new(Mutex::new(1.0)),
        }
    }

    pub fn listen(&self) {
        let game_data_one = Arc::clone(&self.game_data);
        let target_zoom_one = Arc::clone(&self.target_zoom);
        let target_zoom_two = Arc::clone(&self.target_zoom);

        // Listen Keyboard
        thread::spawn(move || Self::listen_keyboard(game_data_one));

        // Spawn a smooth zoom thread
        let zoom_game_data = Arc::clone(&self.game_data);
        thread::spawn(move || Self::smooth_zoom(zoom_game_data, target_zoom_one));

        // Listen Mouse
        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::Wheel { delta_y, .. } => {
                    if delta_y != 0 {
                        let mut target_zoom = target_zoom_two.lock().unwrap();
                        *target_zoom += delta_y as f32 * 0.1;
                        *target_zoom = target_zoom.clamp(0.2, 5.0); // Keep within limits
                    }
                }
                _ => (),
            }
        }) {
            eprintln!("Error: {:?}", error);
        }
    }

    fn smooth_zoom(game_data: Arc<GameData>, target_zoom: Arc<Mutex<f32>>) {
        let steps = 10;
        let step_duration = Duration::from_millis(100) / steps;

        loop {
            let target_zoom = *target_zoom.lock().unwrap();
            game_data.update_field(CAMERA_STATE, |camera| {
                let current_zoom = camera.zoom;
                let zoom_step = (target_zoom - current_zoom) / steps as f32;
                camera.set_zoom(current_zoom + zoom_step);
            });

            thread::sleep(step_duration);
        }
    }

    fn listen_keyboard(game_data: Arc<GameData>) {
        let device_state = DeviceState::new();

        loop {
            let keys = device_state.get_keys();

            if let Some(key_state) = game_data.get_field(KEY_STATE) {
                key_state.w.store(keys.contains(&Keycode::W), Ordering::SeqCst);
                key_state.a.store(keys.contains(&Keycode::A), Ordering::SeqCst);
                key_state.s.store(keys.contains(&Keycode::S), Ordering::SeqCst);
                key_state.d.store(keys.contains(&Keycode::D), Ordering::SeqCst);
            }

            thread::sleep(Duration::from_millis(5));
        }
    }
}
