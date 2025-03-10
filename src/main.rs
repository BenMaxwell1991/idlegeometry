use crate::ui::window::create_window;
use game::data::save_load::{auto_save, load_game_or_new};
use game::loops::game_loop::GameLoop;
use game::loops::input_listener::InputListener;
use std::sync::Arc;
use std::thread;

mod game;
mod ui;
mod enums;

fn main() {
    let game_data = load_game_or_new();
    println!("Initialised all Game Data");

    let game_data_one = Arc::clone(&game_data);
    let game_data_two = Arc::clone(&game_data);
    let game_data_three = Arc::clone(&game_data);
    let game_data_four = Arc::clone(&game_data);
    let game_data_five = Arc::clone(&game_data);

    let game_loop = GameLoop::new(game_data_one);
    let input_listener = InputListener::new(game_data_two);

    thread::spawn(move || game_loop.start_game());
    thread::spawn(move || input_listener.listen());
    thread::spawn(move || auto_save(game_data_three));

    create_window(game_data_five).expect("Failed to start UI");
}
