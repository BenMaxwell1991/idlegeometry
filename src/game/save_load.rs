use crate::game::game::Game;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

const SAVE_FILE: &str = "saved_file";

pub fn load_game_or_new() -> Game {
    if let Ok(save_data) = fs::read_to_string(SAVE_FILE) {
        if let Ok(game) = serde_json::from_str::<Game>(&save_data) {
            println!("Loaded saved game successfully!");
            return game;
        }
    }
    println!("No saved game found, starting a new game.");
    Game::new()
}

pub fn save_game(game: &Arc<Mutex<Game>>) {
    if let Ok(game) = game.lock() {
        if let Ok(serialized) = serde_json::to_string(&*game) {
            if fs::write(SAVE_FILE, serialized).is_ok() {
                println!("Game saved successfully!");
            }
        }
    }
}

pub fn auto_save(game: Arc<Mutex<Game>>) {
    loop {
        let settings = game.lock().unwrap().settings;
        let duration = Duration::from_secs(settings.autosave_interval);
        thread::sleep(duration);
        save_game(&game);
    }
}
