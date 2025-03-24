use crate::enums::gametab::GameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, KEY_STATE, RESOURCES, SETTINGS};
use crate::game::loops::key_state::KeyState;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::objects::animation::Animation;
use crate::game::objects::attacks::attack_defaults::{get_basic_attack, get_modified_attack};
use crate::game::objects::attacks::attack_stats::AttackName;
use crate::game::objects::game_object::{add_units, GameObject};
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use crate::game::objects::unit_defaults::{create_01_baby_dragon, create_02_aqua_drake, create_03_adult_white_dragon};
use crate::game::objects::upgrades::{Upgrade, UpgradeType};
use crate::game::resources::bignumber::BigNumber;
use crate::game::resources::resource::{Resource, DEFAULT_MOVE_SPEED};
use crate::game::settings::Settings;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use crate::ui::asset::sprite::sprite_sheet::{BABY_GREEN_DRAGON, SLASH_ATTACK};
use rand::random_range;
use std::sync::Arc;
use std::time::Duration;

const TILE_SIZE: i32 = 40 * FIXED_POINT_SCALE;
const X_TILE_COUNT: usize = 60;
const Y_TILE_COUNT: usize = 60;
const X_CENTER: i32 = TILE_SIZE * X_TILE_COUNT as i32 / 2;
const Y_CENTER: i32 = TILE_SIZE * Y_TILE_COUNT as i32 / 2;

pub fn init(game_data: GameData) -> GameData {

    // let (steam_client, single) = steamworks::Client::init_app(3585270).expect("Failed to initialize Steam");
    // println!("Logged in as: {}", steam_client.friends().name());
    // *acquire_lock_mut(&game_data.steam_client, "steam_client") = Some(steam_client);

    init_map(&game_data);
    println!("Initialised Map");

    init_attacks(&game_data);
    println!("Initialised Attacks");

    init_player(&game_data);
    println!("Initialised Player");

    init_enemies(&game_data);
    println!("Initialised Enemies");

    init_resources(&game_data);
    println!("Initialised Resources");

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

fn init_resources(game_data: &GameData) {
    {
        let mut resources = game_data.resources.write().unwrap();
        if !resources.contains_key("Gold") {
            resources.insert("Gold".to_string(), 1.0);
            println!("Gold initialized to 1.0");
        }
        if !resources.contains_key("Ruby") {
            resources.insert("Ruby".to_string(), 1.0);
            println!("Ruby initialized to 1.0");
        }
    }
}

fn init_map(game_data: &GameData) {
    *acquire_lock_mut(&game_data.game_map, "game_map") = Some(GameMap::new(X_TILE_COUNT, Y_TILE_COUNT, TILE_SIZE));
    *acquire_lock_mut(&game_data.camera_state, "camera_state") = CameraState::new(Pos2FixedPoint::new(X_CENTER, Y_CENTER), 2048);
}

fn init_attacks(game_data: &GameData) {
    let pool_config = vec![
        (AttackName::Proximity, 2000),
        (AttackName::Swipe, 200),
        (AttackName::FireBolt, 1000),
        (AttackName::LightningBolt, 3000),
    ];

    initialise_attack_pools(game_data, &pool_config);
}

fn init_player(game_data: &GameData) {
    let animation = Animation::new(BABY_GREEN_DRAGON, Duration::from_secs(2), (50, 50));
    let mut player = GameObject::new(ObjectType::Player, ObjectShape::new(40 * FIXED_POINT_SCALE, 40 * FIXED_POINT_SCALE), DEFAULT_MOVE_SPEED, 100.0, 100.0, Some(animation));

    let upgrade = Upgrade {
        upgrade_type: UpgradeType::DecreaseCooldown,
        level: 2,
    };

    for name in [AttackName::Swipe, AttackName::FireBolt, AttackName::LightningBolt] {
        player.attack_cooldowns.insert(name, get_modified_attack(&vec!(upgrade.clone()), name).attack_stats.unwrap().cooldown);
    }

    player.upgrades.push(upgrade);
    player.pickup_radius = Some(300 * FIXED_POINT_SCALE);

    let player_position = Pos2FixedPoint::new(X_CENTER, Y_CENTER);
    add_units(vec![player], vec![player_position], game_data);

    let player_id = game_data.units.read().unwrap()
        .iter()
        .filter_map(|unit_option| unit_option.as_ref())
        .find(|unit| unit.object_type == ObjectType::Player)
        .map(|player| player.id);

    let mut player_id_lock = game_data.player_id.write().unwrap();
    *player_id_lock = player_id;
    let mut player_position_lock = game_data.player_position.write().unwrap();
    *player_position_lock = Some(player_position);
}

fn init_enemies(game_data: &GameData) {
    if let Some(map) = acquire_lock(&game_data.game_map, "game_map").as_ref() {
        let mut units = vec![];
        let mut positions = vec![];

        let map_x = map.width as i32 * map.tile_size;
        let map_y = map.height as i32 * map.tile_size;

        let baby_count = 1000;
        let drake_count = 50;
        let adult_count = 15;

        for _i in 0..baby_count {
            let pos = Pos2FixedPoint::new(random_range(0..=map_x), random_range(0..=map_y));
            let unit = create_01_baby_dragon();
            units.push(unit);
            positions.push(pos);
        }

        for _i in 0..drake_count {
            let pos = Pos2FixedPoint::new(random_range(0..=map_x), random_range(0..=map_y));
            let unit = create_02_aqua_drake();
            units.push(unit);
            positions.push(pos);
        }

        for _i in 0..adult_count {
            let pos = Pos2FixedPoint::new(random_range(0..=map_x), random_range(0..=map_y));
            let unit = create_03_adult_white_dragon();
            units.push(unit);
            positions.push(pos);
        }

        add_units(units, positions, game_data);
    }
}

pub fn initialise_attack_pools(game_data: &GameData, pool_sizes: &[(AttackName, usize)]) {
    let mut attack_pools = game_data.attack_pools.write().unwrap();

    for (attack_name, size) in pool_sizes {
        let mut pool = Vec::with_capacity(*size);
        for _ in 0..*size {
            let attack_unit = get_basic_attack(attack_name.clone());
            pool.push(attack_unit);
        }
        attack_pools.insert(attack_name.clone(), pool);
    }
}