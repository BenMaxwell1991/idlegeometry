use crate::game::game_loop::GameLoop;
use crate::game::save_load::{auto_save, load_game_or_new};
use crate::ui::window::create_window;
use crossbeam::channel;
use std::sync::Arc;
use std::thread;

mod game;
mod ui;
mod resources;
mod enums;

fn main() {
    let (sender, receiver) = channel::bounded(1);
    let game_data = load_game_or_new();
    let game_loop = GameLoop::new(Arc::clone(&game_data));

    let game_data_one = Arc::clone(&game_data);
    let game_data_two = Arc::clone(&game_data);

    thread::spawn(move || game_loop.start_game(sender));
    thread::spawn(move || auto_save(game_data_one));

    create_window(game_data_two, receiver).expect("Failed to start UI");
}
