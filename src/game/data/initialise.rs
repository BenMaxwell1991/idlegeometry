use crate::enums::gametab::GameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, KEY_STATE, PLAYER_POSITION, RESOURCES, SETTINGS};
use crate::game::key_state::KeyState;
use crate::game::settings::Settings;
use crate::resources::bignumber::BigNumber;
use crate::resources::resource::Resource;
use eframe::emath::Pos2;
use std::sync::Arc;

pub fn init(game_data: GameData) -> GameData {

    game_data.set_field(KEY_STATE, Arc::new(KeyState::new()));
    game_data.set_field(CURRENT_TAB, GameTab::default());
    game_data.set_field(PLAYER_POSITION, Pos2::new(100.0, 100.0));

    if game_data.get_field(RESOURCES).is_none() {
        println!("No saved game found, starting a new game.");
        game_data.set_field(RESOURCES, vec![
            Resource::new("Points", BigNumber::new(0.0), BigNumber::new(0.03), BigNumber::new(0.0), BigNumber::new(0.0), true),
            Resource::with_defaults("Lines"),
            Resource::with_defaults("Triangles"),
        ]);
    }

    if game_data.get_field(SETTINGS).is_none() {
        game_data.set_field(SETTINGS, Settings::default());
    }

    game_data
}