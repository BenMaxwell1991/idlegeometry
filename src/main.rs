use crate::game::game::Game;
use crate::game::save_load::{auto_save, load_game_or_new};
use crate::ui::window::create_window;
use crossbeam::channel;
use std::sync::{Arc, Mutex};
use std::thread;

mod game;
mod ui;
mod resources;
mod enums;

fn main() {
    let game = Arc::new(Mutex::new(load_game_or_new()));
    let (sender, receiver) = channel::bounded(1);

    let game_ref_one = Arc::clone(&game);
    let game_ref_two = Arc::clone(&game);
    let game_ref_three = Arc::clone(&game);

    thread::spawn(move || Game::start_game(game_ref_one, sender));
    thread::spawn(move || auto_save(game_ref_two));
    create_window(game_ref_three, receiver).expect("Failed to start UI");
}