use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::KEY_STATE;
use device_query_revamped::{DeviceQuery, DeviceState, Keycode};
use std::sync::Arc;
use std::time::Duration;

pub struct InputListener {
    game_data: Arc<GameData>,
}

impl InputListener {
    pub fn new(game_data: Arc<GameData>) -> Self {
        Self { game_data }
    }

    pub fn listen(&self) {
        let device_state = DeviceState::new();

        if let Some(key_state) = self.game_data.get_field(KEY_STATE) {
            loop {
                let keys = device_state.get_keys();

                key_state.w.store(keys.contains(&Keycode::W), std::sync::atomic::Ordering::SeqCst);
                key_state.a.store(keys.contains(&Keycode::A), std::sync::atomic::Ordering::SeqCst);
                key_state.s.store(keys.contains(&Keycode::S), std::sync::atomic::Ordering::SeqCst);
                key_state.d.store(keys.contains(&Keycode::D), std::sync::atomic::Ordering::SeqCst);

                std::thread::sleep(Duration::from_millis(1));
            }
        } else {
            eprintln!("Error: KeyState not found in GameData!");
        }
    }
}
