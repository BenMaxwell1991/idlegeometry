use crate::game::game::Game;
use crate::ui::window::create_window;
use crossbeam::channel;
use std::sync::{Arc, Mutex};
use std::thread;

mod game;
mod constants;
mod ui;
mod resources;
mod enums;

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));
    let (sender, receiver) = channel::bounded(1);

    let game_clone = Arc::clone(&game);
    thread::spawn(move || {
        Game::start_game(game_clone, sender);
    });

    create_window(game, receiver).expect("Failed to start UI");

}