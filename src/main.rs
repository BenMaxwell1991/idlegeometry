use crate::game::data::initialise::initialise_sound_pools;
use crate::ui::sound::music_player::start_music_thread;
use crate::ui::window::create_window;
use game::data::save_load::{auto_save, load_game_or_new};
use game::loops::game_loop::GameLoop;
use game::loops::input_listener::InputListener;
use rayon::{current_num_threads, ThreadPoolBuilder};
use rodio::OutputStream;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use num_cpus::get;
use rayon::prelude::ParallelSliceMut;
use crate::enums::gametab::GameTab;
use crate::enums::gametab::GameTab::NullGameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, GAME_IN_FOCUS, KEY_STATE};
use crate::game::loops::game_loop::get_player_position;
use crate::game::loops::key_state::KeyState;
use crate::game::loops::move_player::move_player;
use crate::game::maths::pos_2::Pos2FixedPoint;
use crate::game::units::unit_type::UnitType;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};

mod game;
mod ui;
mod enums;
mod helper;

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build_global()
        .unwrap();

    let game_data = load_game_or_new();
    println!("Initialised all Game Data");

    let (stream, stream_handle) = OutputStream::try_default().expect("❌ Failed to create audio output stream");
    *game_data.audio_stream_handle.write().unwrap() = Some(stream_handle);
    initialise_sound_pools(&game_data);
    println!("Initialised Sounds");

    let game_data_one = Arc::clone(&game_data);
    let game_data_two = Arc::clone(&game_data);
    let game_data_three = Arc::clone(&game_data);
    let game_data_four = Arc::clone(&game_data);
    let game_data_five = Arc::clone(&game_data);
    let game_data_six = Arc::clone(&game_data);
    let game_data_seven = Arc::clone(&game_data);

    let game_loop = GameLoop::new(game_data_one);
    let input_listener = InputListener::new(game_data_two);

    thread::spawn(move || game_loop.start_game());
    thread::spawn(move || move_player(game_data_five));
    thread::spawn(move || input_listener.listen());
    thread::spawn(move || auto_save(game_data_three));

    start_music_thread(game_data_four);

    create_window(game_data_six).expect("Failed to start UI");
}