use crate::game::game_loop::GameLoop;
use crate::game::input_listener::InputListener;
use crate::ui::window::create_window;
use crossbeam::channel;
use game::data::save_load::{auto_save, load_game_or_new};
use std::sync::Arc;
use std::thread;

mod game;
mod ui;
mod resources;
mod enums;

fn main() {
    let (sender, receiver) = channel::bounded(1);

    let game_data = load_game_or_new();

    // let (steam_client, single) = steamworks::Client::init_app(480).expect("Failed to initialize Steam");
    // println!("Logged in as: {}", steam_client.friends().name());
    // steam_client.friends().set_rich_presence("status", Some("Developing My Game!"));
    // game_data.set_steam_client(steam_client);

    let game_data_one = Arc::clone(&game_data);
    let game_data_two = Arc::clone(&game_data);
    let game_data_three = Arc::clone(&game_data);
    let game_data_four = Arc::clone(&game_data);

    let game_loop = GameLoop::new(game_data_one);
    let input_listener = InputListener::new(game_data_two);

    thread::spawn(move || game_loop.start_game(sender));
    thread::spawn(move || input_listener.listen());
    thread::spawn(move || auto_save(game_data_three));

    create_window(game_data_four, receiver).expect("Failed to start UI");
}
