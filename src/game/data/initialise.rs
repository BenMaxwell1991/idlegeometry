use crate::enums::gametab::GameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CAMERA_STATE, CURRENT_TAB, GAME_MAP, KEY_STATE, PLAYER_POSITION, RESOURCES, SETTINGS, UNITS};
use crate::game::loops::key_state::KeyState;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::resources::bignumber::BigNumber;
use crate::game::resources::resource::{Resource, DEFAULT_HEALTH, DEFAULT_MANA, DEFAULT_MOVE_SPEED};
use crate::game::settings::Settings;
use crate::game::units::animation::Animation;
use crate::game::units::unit::Unit;
use crate::game::units::unit_type::UnitType;
use crate::ui::asset::sprite::sprite_sheet::BABY_GREEN_DRAGON;
use eframe::emath::Pos2;
use std::sync::Arc;
use std::time::Duration;

const TILE_SIZE: f32 = 40.0;
const X_TILE_COUNT: usize = 50;
const Y_TILE_COUNT: usize = 50;
const X_CENTER: f32 = TILE_SIZE * X_TILE_COUNT as f32 / 2.0;
const Y_CENTER: f32 = TILE_SIZE * Y_TILE_COUNT as f32 / 2.0;

pub fn init(game_data: GameData) -> GameData {

    // let (steam_client, single) = steamworks::Client::init_app(480).expect("Failed to initialize Steam");
    // println!("Logged in as: {}", steam_client.friends().name());
    // game_data.set_field(STEAM_CLIENT, steam_client);

    init_map(&game_data);
    init_player(&game_data);
    game_data.set_field(KEY_STATE, Arc::new(KeyState::new()));
    game_data.set_field(CURRENT_TAB, GameTab::default());

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

fn init_map(game_data: &GameData) {
    game_data.set_field(GAME_MAP, GameMap::new(X_TILE_COUNT, Y_TILE_COUNT, TILE_SIZE));
    game_data.set_field(PLAYER_POSITION, Pos2::new(X_CENTER, Y_CENTER));
    game_data.set_field(CAMERA_STATE, CameraState::new(Pos2::new(X_CENTER, Y_CENTER), 1.0));

}

fn init_player(game_data: &GameData) {
    let stats = vec!(DEFAULT_MOVE_SPEED.clone(), DEFAULT_HEALTH.clone(), DEFAULT_MANA.clone());
    let animation = Animation::new(BABY_GREEN_DRAGON, Duration::from_secs(1));
    let units = vec!(Unit::new(UnitType::Player, Pos2::new(X_CENTER, Y_CENTER), stats, animation));
    game_data.set_field(UNITS, units)
}