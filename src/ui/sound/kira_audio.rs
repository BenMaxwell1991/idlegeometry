use kira::sound::static_sound::StaticSoundData;
use kira::sound::streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings};
use kira::sound::{FromFileError, PlaybackState, SoundData};
use kira::{AudioManager, AudioManagerSettings};
use rustc_hash::FxHashMap;
use std::io::Cursor;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

pub const TRACK_01: &str = "01_midnight_wander";
pub const TRACK_02: &str = "02_chilled_guitar";
pub const TRACK_03: &str = "03_epic_orchestral";
pub const TRACK_04: &str = "04_epic_electric";
pub const SOUND_01: &str = "01_sound_collect_ruby";

pub const MIDNIGHT_WANDER: &[u8] = include_bytes!("../asset/sound/soundtrack/01_midnight_wander.mp3");
pub const CHILL_GUITAR: &[u8] = include_bytes!("../asset/sound/soundtrack/02_chilled_guitar.mp3");
pub const EPIC_ORCHESTRAL: &[u8] = include_bytes!("../asset/sound/soundtrack/03_epic_orchestral.mp3");
pub const EPIC_ELECTRIC: &[u8] = include_bytes!("../asset/sound/soundtrack/04_epic_electric.mp3");
pub const RUBY_SOUND: &[u8] = include_bytes!("../asset/sound/pickup/sound_collect_ruby.mp3");

pub const SOUNDTRACKS: [(&str, &[u8]); 4] = [
    (TRACK_01, MIDNIGHT_WANDER),
    (TRACK_02, CHILL_GUITAR),
    (TRACK_03, EPIC_ORCHESTRAL),
    (TRACK_04, EPIC_ELECTRIC),
];

pub const SOUND_EFFECTS: [(&str, &[u8]); 1] = [
    (SOUND_01, RUBY_SOUND),
];

pub struct KiraAudio {
    audio_manager: AudioManager,
    sound_effects: FxHashMap<String, StaticSoundData>,
    soundtracks: FxHashMap<String, StreamingSoundData<FromFileError>>,
    current_soundtrack: Option<StreamingSoundHandle<FromFileError>>,
}

impl KiraAudio {
    pub fn new() -> Self {
        let mut manager = AudioManager::new(AudioManagerSettings::default()).unwrap_or_else(|e| {
            eprintln!("Failed to initialize audio manager: {}", e);
            std::process::exit(1);
        });

        // Load sound effects (static)
        let mut sound_effects = FxHashMap::default();
        for (name, bytes) in SOUND_EFFECTS {
            let sound_data = StaticSoundData::from_cursor(Cursor::new(bytes)).unwrap_or_else(|e| {
                eprintln!("Failed to load static sound effect {}: {}", name, e);
                std::process::exit(1);
            });
            sound_effects.insert(name.to_string(), sound_data);
        }

        // Load soundtracks (streaming)
        let mut soundtracks = FxHashMap::default();
        for (name, bytes) in SOUNDTRACKS {
            let stream_data = StreamingSoundData::from_cursor(Cursor::new(bytes))
                .unwrap_or_else(|e| {
                    eprintln!("Failed to load streaming soundtrack {}: {}", name, e);
                    exit(1);
                })
                .with_settings(StreamingSoundSettings::new().loop_region(..));

            soundtracks.insert(name.to_string(), stream_data);
        }

        Self {
            audio_manager: manager,
            sound_effects,
            soundtracks,
            current_soundtrack: None,
        }
    }

    pub fn play_sound(&mut self, name: &str) {
        if let Some(sound_data) = self.sound_effects.get(name) {
            if let Err(e) = self.audio_manager.play(sound_data.clone()) {
                eprintln!("Failed to play sound '{}': {}", name, e);
            }
        } else {
            eprintln!("Sound effect '{}' not found!", name);
        }
    }

    pub fn play_soundtrack_looping(&mut self) {
        if let Some(track) = self.soundtracks.remove(TRACK_02) {
            println!("Trying to play");
            let handle = self.audio_manager.play(track).expect("TODO: panic message");
            println!("Played");

            while handle.state() == PlaybackState::Playing {
                sleep(Duration::from_millis(100)); // Check every 100ms
            }

        } else {
            eprintln!("Soundtrack '{}' not found!", TRACK_02);
        }
    }
}