use crate::game::game_data::GameData;
use crate::game::settings::Settings;
use crate::resources::bignumber::BigNumber;
use crate::resources::resource::Resource;
use serde_json::Value;
use std::fs;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const SAVE_FILE: &str = "saved_file";

pub fn load_game_or_new() -> Arc<GameData> {
    let game_data = GameData::new();

    if let Ok(save_data) = fs::read_to_string(SAVE_FILE) {
        if let Ok(json_data) = serde_json::from_str::<Value>(&save_data) {
            if let Some(resources) = json_data.get("resources")
                .and_then(|v| serde_json::from_value::<Vec<Resource>>(v.clone()).ok()) {
                game_data.set_field("resources", resources);
                println!("Loaded resources successfully!");
            }

            if let Some(settings) = json_data.get("settings")
                .and_then(|v| serde_json::from_value::<Settings>(v.clone()).ok()) {
                game_data.set_field("settings", settings);
                println!("Loaded settings successfully!");
            }
        }
    }

    // If data wasn't loaded, initialize default values
    if game_data.get_field::<Vec<Resource>>("resources").is_none() {
        println!("No saved game found, starting a new game.");
        game_data.set_field("resources", vec![
            Resource::new("Points", BigNumber::new(0.0), BigNumber::new(0.01), BigNumber::new(0.0), true),
            Resource::with_defaults("Lines"),
            Resource::with_defaults("Triangles"),
        ]);
    }

    if game_data.get_field::<Settings>("settings").is_none() {
        game_data.set_field("settings", Settings::default());
    }

    Arc::new(game_data)
}

pub fn save_game(game_data: &Arc<GameData>) {
    let mut save_map = serde_json::Map::new();

    if let Some(resources) = game_data.get_field::<Vec<Resource>>("resources") {
        if let Ok(serialized) = serde_json::to_value(&resources) {
            save_map.insert("resources".to_string(), serialized);
        }
    }

    if let Some(settings) = game_data.get_field::<Settings>("settings") {
        if let Ok(serialized) = serde_json::to_value(&settings) {
            save_map.insert("settings".to_string(), serialized);
        }
    }

    if !save_map.is_empty() {
        if let Ok(serialized_data) = serde_json::to_string_pretty(&serde_json::Value::Object(save_map)) {
            if fs::write(SAVE_FILE, serialized_data).is_ok() {
                println!("Game saved successfully {}", chrono::Local::now().to_rfc2822());
            }
        }
    }
}

pub fn auto_save(game_data: Arc<GameData>) {
    loop {
        let autosave_interval = game_data.get_field::<Settings>("settings")
            .map(|s| s.autosave_interval)
            .unwrap_or(5);

        thread::sleep(Duration::from_secs(autosave_interval));
        save_game(&game_data);
    }
}