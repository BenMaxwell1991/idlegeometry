use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use rodio::{Decoder, Sink};
use std::io::Cursor;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// ✅ Sound Names
pub const MIDNIGHT_WANDER: &str = "midnight_wander";
pub const ATTACK_SWIPE_01: &str = "attack_swipe_01";
pub const ATTACK_SWIPE_02: &str = "attack_swipe_02";


pub const MIDNIGHT_WANDER_BYTES: &[u8] = include_bytes!("../asset/sound/soundtrack/01_midnight_wander.mp3");
pub const ATTACK_SWIPE_01_BYTES: &[u8] = include_bytes!("../asset/sound/attack/swipe/attack_swipe_01.mp3");
pub const ATTACK_SWIPE_02_BYTES: &[u8] = include_bytes!("../asset/sound/attack/swipe/attack_swipe_02.mp3");

/// ✅ All Sounds in One Array
pub const SOUND_FILES: [(&str, &[u8]); 3] = [
    (MIDNIGHT_WANDER, MIDNIGHT_WANDER_BYTES),
    (ATTACK_SWIPE_01, ATTACK_SWIPE_01_BYTES),
    (ATTACK_SWIPE_02, ATTACK_SWIPE_02_BYTES),
];


pub fn start_music_thread(game_data: Arc<GameData>) {
    thread::spawn(move || {
        loop {
            let game_state = *game_data.game_state.read().unwrap();

            let track_name = if game_state == GameState::Playing {
                Some(MIDNIGHT_WANDER.to_string())
            } else {
                None
            };

            {
                let mut current_track = game_data.current_track.write().unwrap();
                let mut active_sounds = game_data.active_sounds.write().unwrap();

                if track_name.is_none() {
                    let active_sounds_clone = Arc::clone(&game_data.active_sounds);
                    thread::spawn(move || {
                        let mut active_sounds = active_sounds_clone.write().unwrap();
                        for sink in active_sounds.iter() {
                            smooth_set_volume(sink, 0.0, 50, 300);
                            sink.pause();
                        }
                    });

                    *current_track = None;
                }

                else if Some(track_name.clone().unwrap()) != *current_track {
                    println!("🎵 Starting new music: {}", track_name.as_ref().unwrap());
                    play_sound(Arc::clone(&game_data), &track_name.clone().unwrap());
                    *current_track = track_name;
                }

                active_sounds.retain(|sink| !sink.empty());
            }

            sleep(Duration::from_millis(250));
        }
    });
}

pub fn play_sound(game_data: Arc<GameData>, sound_name: &str) {
    if let Some(stream_handle) = game_data.audio_stream_handle.read()
        .expect("❌ Failed to acquire read lock on audio stream")
        .as_ref()
    {
        let sound_bytes = match sound_name {
            MIDNIGHT_WANDER => Some(MIDNIGHT_WANDER_BYTES),
            ATTACK_SWIPE_01 => Some(ATTACK_SWIPE_01_BYTES),
            ATTACK_SWIPE_02 => Some(ATTACK_SWIPE_02_BYTES),
            _ => None,
        };

        if let Some(bytes) = sound_bytes {
            let game_data_clone = Arc::clone(&game_data);
            let stream_handle = stream_handle.clone();
            let sound_name = sound_name.to_string();

            thread::spawn(move || {
                if let Ok(source) = Decoder::new(Cursor::new(bytes)) {
                    let sink = Sink::try_new(&stream_handle)
                        .expect("❌ Failed to create audio sink");

                    sink.append(source);
                    sink.play();

                    let mut active_sounds = game_data_clone.active_sounds.write().unwrap();
                    active_sounds.push(sink);

                    println!("✅ Playing sound asynchronously: {}", sound_name);
                }
            });
        } else {
            eprintln!("❌ Sound not found: {}", sound_name);
        }
    }
}

fn smooth_set_volume(sink: &Sink, target_volume: f32, steps: usize, duration_ms: u64) {
    let current_volume = sink.volume();
    let step_size = (current_volume - target_volume) / steps as f32;

    for i in 0..steps {
        let new_volume = current_volume - (step_size * i as f32);
        sink.set_volume(new_volume);
        sleep(Duration::from_millis(duration_ms / steps as u64));
    }

    sink.set_volume(target_volume);
}