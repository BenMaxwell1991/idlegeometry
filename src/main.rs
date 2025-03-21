use crate::ui::sound::kira_audio::{KiraAudio};
use crate::ui::window::create_window;
use game::data::save_load::{auto_save, load_game_or_new};
use game::loops::game_loop::GameLoop;
use game::loops::input_listener::InputListener;
use rayon::prelude::ParallelSliceMut;
use rayon::ThreadPoolBuilder;
use std::sync::Arc;
use std::thread;

mod game;
mod ui;
mod enums;
mod helper;

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build_global()
        .unwrap();

    println!("0");
    let game_data = load_game_or_new();
    println!("1");
    let mut testing = KiraAudio::new();
    println!("Initialised all Game Data");

    let game_data_arc = Arc::new(game_data);
    let game_data_one = Arc::clone(&game_data_arc);
    let game_data_two = Arc::clone(&game_data_arc);
    let game_data_three = Arc::clone(&game_data_arc);
    let game_data_four = Arc::clone(&game_data_arc);
    let game_data_five = Arc::clone(&game_data_arc);

    let game_loop = GameLoop::new(game_data_one);
    println!("2");
    let input_listener = InputListener::new(game_data_two);
    println!("3");

    thread::spawn(move || game_loop.start_game());
    thread::spawn(move || input_listener.listen());
    thread::spawn(move || auto_save(game_data_three));
    thread::spawn(move || { testing.play_soundtrack_looping(); });

    create_window(game_data_five).expect("Failed to start UI");
}