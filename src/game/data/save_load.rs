use crate::game::data::game_data::GameData;
use crate::game::data::initialise_data::initialise_data;
use crate::game::data::stored_data::SETTINGS;
use crate::game::settings::Settings;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use chrono::Local;
use rustc_hash::FxHashMap;
use serde_json::{from_str, from_value, to_string_pretty, to_value, Map, Value};
use std::fs;
use std::fs::read_to_string;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crate::game::data::player_data::PlayerData;

const SAVE_FILE: &str = "saved_file";

pub fn load_game_or_new() -> GameData {
    let game_data = GameData::new();

    if let Ok(save_data) = read_to_string(SAVE_FILE) {
        if let Ok(json_data) = from_str::<Value>(&save_data) {
            if let Some(player_data) = json_data.get("player_data")
                .and_then(|v| from_value::<PlayerData>(v.clone()).ok()) {
                *acquire_lock_mut(&game_data.player_data, "player_data") = player_data;
                println!("Loaded player data successfully!");
            }

            if let Some(settings) = json_data.get("settings")
                .and_then(|v| from_value::<Settings>(v.clone()).ok()) {
                game_data.set_field(SETTINGS, settings);
                println!("Loaded settings successfully!");
            }
        }
    }

    initialise_data(game_data)
}

pub fn save_game(game_data: &Arc<GameData>) {
    let mut save_map = Map::new();

    if let player_data = acquire_lock(&game_data.player_data, "player_data").clone() {
        if let Ok(serialized) = to_value(&player_data) {
            save_map.insert("player_data".to_string(), serialized);
        }
    }

    if let Some(settings) = game_data.get_field(SETTINGS) {
        if let Ok(serialized) = to_value(&settings) {
            save_map.insert("settings".to_string(), serialized);
        }
    }

    if !save_map.is_empty() {
        if let Ok(serialized_data) = to_string_pretty(&Value::Object(save_map)) {
            if fs::write(SAVE_FILE, serialized_data).is_ok() {
                println!("Game saved successfully {}", Local::now().to_rfc2822());
            }
        }
    }
}

pub fn auto_save(game_data: Arc<GameData>) {
    loop {
        let autosave_interval = game_data.get_field(SETTINGS)
            .map(|s| s.autosave_interval)
            .unwrap_or(5);

        sleep(Duration::from_secs(autosave_interval));
        save_game(&game_data);
    }
}