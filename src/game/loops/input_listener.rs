use std::collections::HashSet;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::KEY_STATE;
use device_query_revamped::{DeviceQuery, DeviceState, Keycode};
use rdev::{listen, EventType};
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct InputListener {
    game_data: Arc<GameData>,
    target_zoom: Arc<Mutex<i32>>,
}

impl InputListener {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self {
            game_data,
            target_zoom: Arc::new(Mutex::new(1_024)),
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
                        *target_zoom += (delta_y as i32) << 7;
                        *target_zoom = target_zoom.clamp(256, 4_096); // Keep within limits
                    }
                }
                _ => (),
            }
        }) {
            eprintln!("Error: {:?}", error);
        }
    }

    fn smooth_zoom(game_data: Arc<GameData>, target_zoom: Arc<Mutex<i32>>) {
        let steps_bits = 3;
        let step_duration = Duration::from_millis(100 >> steps_bits);

        loop {
            let target_zoom = *target_zoom.lock().unwrap();

            {
                let mut camera_state = game_data.camera_state.write().unwrap();
                let current_zoom = camera_state.zoom;
                let zoom_step = (target_zoom - current_zoom) >> steps_bits;
                camera_state.set_zoom(current_zoom + zoom_step);
            }


            thread::sleep(step_duration);
        }
    }

    fn listen_keyboard(game_data: Arc<GameData>) {
        let device_state = DeviceState::new();
        let mut last_pressed = Vec::new();

        loop {
            let keys = device_state.get_keys();
            let mut key_queue = game_data.key_queue.write().unwrap();

            if let Some(key_state) = game_data.get_field(KEY_STATE) {
                key_state.w.store(keys.contains(&Keycode::W), Ordering::SeqCst);
                key_state.a.store(keys.contains(&Keycode::A), Ordering::SeqCst);
                key_state.s.store(keys.contains(&Keycode::S), Ordering::SeqCst);
                key_state.d.store(keys.contains(&Keycode::D), Ordering::SeqCst);
            }

            for &key in &keys {
                if !last_pressed.contains(&key) {
                    println!("Key pressed: {:?}", key);
                    key_queue.push(key);
                }
            }

            last_pressed = keys.clone();

            drop(key_queue);
            thread::sleep(Duration::from_millis(10));
        }
    }

}
