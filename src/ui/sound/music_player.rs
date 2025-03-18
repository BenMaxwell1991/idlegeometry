use crate::enums::gamestate::GameState;
use crate::game::data::game_data::GameData;
use rodio::{Decoder, Sink, Source};
use std::io::Cursor;
use std::sync::Arc;
use std::thread;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};
use crate::helper::lock_helper::acquire_lock_mut;

/// ✅ Sound Names
pub const MIDNIGHT_WANDER: &str = "midnight_wander";
pub const ATTACK_SWIPE_01: &str = "attack_swipe_01";
pub const ATTACK_SWIPE_02: &str = "attack_swipe_02";
pub const COLLECT_RUBY: &str = "collect_ruby";
pub const SELL_GOLD: &str = "sell_gold";
pub const MONSTER_DEATH_01: &str = "monster_death_01";


pub const MIDNIGHT_WANDER_BYTES: &[u8] = include_bytes!("../asset/sound/soundtrack/01_midnight_wander.mp3");
pub const ATTACK_SWIPE_01_BYTES: &[u8] = include_bytes!("../asset/sound/attack/swipe/attack_swipe_01.mp3");
pub const ATTACK_SWIPE_02_BYTES: &[u8] = include_bytes!("../asset/sound/attack/swipe/attack_swipe_02.mp3");
pub const COLLECT_RUBY_BYTES: &[u8] = include_bytes!("../asset/sound/pickup/sound_collect_ruby.mp3");
pub const SELL_GOLD_BYTES: &[u8] = include_bytes!("../asset/sound/pickup/sound_sell_item.mp3");
pub const MONSTER_DEATH_01_BYTES: &[u8] = include_bytes!("../asset/sound/pickup/sound_death_01.mp3");

/// ✅ All Sounds in One Array
pub const SOUND_FILES: [(&str, &[u8], u32); 6] = [
    (MIDNIGHT_WANDER, MIDNIGHT_WANDER_BYTES, 1),
    (ATTACK_SWIPE_01, ATTACK_SWIPE_01_BYTES, 3),
    (ATTACK_SWIPE_02, ATTACK_SWIPE_02_BYTES, 3),
    (COLLECT_RUBY, COLLECT_RUBY_BYTES, 4),
    (SELL_GOLD, SELL_GOLD_BYTES, 4),
    (MONSTER_DEATH_01, MONSTER_DEATH_01_BYTES, 4),
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
                let mut active_sounds = game_data.active_sounds.write().unwrap();
                let mut current_track = game_data.current_track.write().unwrap();

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
                    play_sound(Arc::clone(&game_data), &track_name.clone().unwrap(), 1.0);
                    *current_track = track_name;
                }

                active_sounds.retain(|sink| !sink.empty());
            }

            sleep(Duration::from_millis(250));
        }
    });
}

pub fn play_sound(game_data: Arc<GameData>, sound_name: &str, volume: f32) {
    let volume = volume.clamp(0.0, 1.0);
    let mut sound_pools = game_data.sound_pools.write().expect("❌ Failed to acquire write lock on sound pools");

    if let Some((_, sound_bytes, _)) = SOUND_FILES.iter().find(|(name, _, _)| *name == sound_name) {
        if let Some(pool) = sound_pools.get_mut(sound_name) {
            if let Some(sink) = pool.pop_front() {
                let game_data_clone = Arc::clone(&game_data);
                let sound_name = sound_name.to_string();

                spawn(move || {
                    if let Ok(source) = Decoder::new(Cursor::new(sound_bytes)) {
                        sink.set_volume(volume);
                        sink.append(source.convert_samples::<f32>());
                        sink.sleep_until_end();

                        let mut sound_pools = acquire_lock_mut(&game_data_clone.sound_pools, "❌ Failed to acquire write lock on sound pools");
                        if let Some(pool) = sound_pools.get_mut(&sound_name) {
                            pool.push_back(sink);
                        }
                    }
                });
            }
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