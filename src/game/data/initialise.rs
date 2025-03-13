use crate::enums::gametab::GameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{ATTACKS, CAMERA_STATE, CURRENT_TAB, GAME_MAP, KEY_STATE, PLAYER_POSITION, RESOURCES, SETTINGS};
use crate::game::loops::key_state::KeyState;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::resources::bignumber::BigNumber;
use crate::game::resources::resource::{Resource, DEFAULT_MOVE_SPEED, DEFAULT_STATS};
use crate::game::settings::Settings;
use crate::game::units::animation::Animation;
use crate::game::units::attack::Attack;
use crate::game::units::create_units::create_enemy_at_point;
use crate::game::units::unit::{add_units, Unit};
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use crate::ui::asset::sprite::sprite_sheet::{BABY_GREEN_DRAGON, SLASH_ATTACK, YOUNG_RED_DRAGON};
use rand::random_range;
use std::sync::Arc;
use std::time::Duration;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};

const TILE_SIZE: i32 = 40 * FIXED_POINT_SCALE;
const X_TILE_COUNT: usize = 50;
const Y_TILE_COUNT: usize = 50;
const X_CENTER: i32 = TILE_SIZE * X_TILE_COUNT as i32 / 2;
const Y_CENTER: i32 = TILE_SIZE * Y_TILE_COUNT as i32 / 2;

pub fn init(game_data: GameData) -> GameData {

    // let (steam_client, single) = steamworks::Client::init_app(480).expect("Failed to initialize Steam");
    // println!("Logged in as: {}", steam_client.friends().name());
    // game_data.set_field(STEAM_CLIENT, steam_client);

    init_map(&game_data);
    println!("Initialised Map");

    init_attacks(&game_data);
    println!("Initialised Attacks");

    init_player(&game_data);
    println!("Initialised Player");

    init_enemies(&game_data);
    println!("Initialised Enemies");

    game_data.set_field(KEY_STATE, Arc::new(KeyState::new()));
    game_data.set_field(CURRENT_TAB, GameTab::default());

    if game_data.get_field(RESOURCES).is_none() {
        println!("No saved game found, starting a new game.");
        game_data.set_field(RESOURCES, vec![
            Resource::new("Points", BigNumber::new(0.0), BigNumber::new(0.03), BigNumber::new(0.0), BigNumber::new(0.0), true),
            Resource::with_defaults("Lines"),
            Resource::with_defaults("Triangles"),
        ]);
        println!("Initialised Resources");
    }

    if game_data.get_field(SETTINGS).is_none() {
        println!("Initialised Settings");
        game_data.set_field(SETTINGS, Settings::default());
    }

    game_data
}

fn init_map(game_data: &GameData) {
    game_data.set_field(GAME_MAP, GameMap::new(X_TILE_COUNT, Y_TILE_COUNT, TILE_SIZE));
    game_data.set_field(PLAYER_POSITION, Pos2FixedPoint::new(X_CENTER, Y_CENTER));
    game_data.set_field(CAMERA_STATE, CameraState::new(Pos2FixedPoint::new(X_CENTER, Y_CENTER), 256));
}

fn init_attacks(game_data: &GameData) {
    let mut attacks = Vec::new();

    let animation = Animation::new(SLASH_ATTACK, Duration::from_millis(1000));
    let attack = Attack::new("slash", 10.0, 5.0, 2.0, 0.0, 0.0, 0.0, animation, Default::default(), true);

    attacks.push(attack);

    game_data.set_field(ATTACKS, attacks);
}

fn init_player(game_data: &GameData) {
    let animation = Animation::new(BABY_GREEN_DRAGON, Duration::from_secs(1));
    let mut player = Unit::new(UnitType::Player, UnitShape::new(16 * FIXED_POINT_SCALE, 16 * FIXED_POINT_SCALE), DEFAULT_MOVE_SPEED, DEFAULT_STATS.clone(), animation);

    if let Some(attack) = game_data.get_field(ATTACKS).unwrap().iter().find(|attack| attack.name == SLASH_ATTACK) {
        player.attacks.push(attack.clone());
    }

    add_units(vec![player], vec![Pos2FixedPoint::new(X_CENTER, Y_CENTER)], game_data);
}

fn init_enemies(game_data: &GameData) {
    if let Some(map) = game_data.get_field(GAME_MAP) {
        let mut units = vec![];
        let mut positions = vec![];

        let map_x = map.width as i32 * map.tile_size;
        let map_y = map.height as i32 * map.tile_size;

        for _i in 0..9999 {
            let pos = Pos2FixedPoint::new(random_range(0..=map_x), random_range(0..=map_y));
            units.push(create_enemy_at_point(YOUNG_RED_DRAGON));
            positions.push(pos);
        }

        add_units(units, positions, game_data);
    }
}
