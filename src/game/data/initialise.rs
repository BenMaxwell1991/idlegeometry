use crate::enums::gametab::GameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, KEY_STATE, RESOURCES, SETTINGS};
use crate::game::loops::key_state::KeyState;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::resources::bignumber::BigNumber;
use crate::game::resources::resource::{Resource, DEFAULT_MOVE_SPEED};
use crate::game::settings::Settings;
use crate::game::units::animation::Animation;
use crate::game::units::attack::{Attack, AttackName};
use crate::game::units::unit::{add_units, Unit};
use crate::game::units::unit_defaults::{create_01_baby_dragon, create_02_aqua_drake, create_03_adult_white_dragon};
use crate::game::units::unit_shape::UnitShape;
use crate::game::units::unit_type::UnitType;
use crate::game::units::upgrades::{Upgrade, UpgradeType};
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use crate::ui::asset::sprite::sprite_sheet::{BABY_GREEN_DRAGON, SLASH_ATTACK};
use crate::ui::sound::music_player::SOUND_FILES;
use rand::random_range;
use rodio::Sink;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

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
    let animation = Animation::new(SLASH_ATTACK, Duration::from_millis(1000), (70, 70));
    let animation = Animation::new(BABY_GREEN_DRAGON, Duration::from_millis(1000), (70, 70));

    let pool_config = vec![
        (AttackName::Swipe, animation.clone(), 1000), // Up to 1000 Swipes available
        (AttackName::Fireball, animation.clone(), 1000), // Up to 1000 Swipes available
    ];

    initialise_attack_pools(game_data, &pool_config);
}

fn init_player(game_data: &GameData) {
    let animation = Animation::new(BABY_GREEN_DRAGON, Duration::from_secs(1), (50, 50));
    let mut player = Unit::new(UnitType::Player, UnitShape::new(40 * FIXED_POINT_SCALE, 40 * FIXED_POINT_SCALE), DEFAULT_MOVE_SPEED, 10.0, 5.0, animation);

    let upgrade = Upgrade {
        upgrade_type: UpgradeType::DecreaseCooldown,
        level: 15,
    };

    player.attack_cooldowns.insert(AttackName::Swipe, 2.0);
    player.upgrades.push(upgrade);
    player.pickup_radius = Some(300 * FIXED_POINT_SCALE);

    add_units(vec![player], vec![Pos2FixedPoint::new(X_CENTER, Y_CENTER)], game_data);

    let player_id = game_data.units.read().unwrap()
        .iter()
        .filter_map(|unit_option| unit_option.as_ref())
        .find(|unit| unit.unit_type == UnitType::Player)
        .map(|player| player.id);

    let mut player_id_lock = game_data.player_id.write().unwrap();
    *player_id_lock = player_id;
}

fn init_enemies(game_data: &GameData) {
    if let Some(map) = acquire_lock(&game_data.game_map, "game_map").as_ref() {
        let mut units = vec![];
        let mut positions = vec![];

        let map_x = map.width as i32 * map.tile_size;
        let map_y = map.height as i32 * map.tile_size;

        let baby_count = 2500;
        let drake_count = 100;
        let adult_count = 20;

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

pub fn initialise_attack_pools(game_data: &GameData, pool_sizes: &[(AttackName, Animation, usize)]) {
    let mut attack_pools = game_data.attack_pools.write().unwrap();

    for (attack_name, animation, size) in pool_sizes {
        let mut pool = Vec::with_capacity(*size);
        for _ in 0..*size {
            pool.push(Attack::get_basic_attack(attack_name.clone()));
        }
        attack_pools.insert(attack_name.clone(), pool);
    }
}

pub fn initialise_sound_pools(game_data: &GameData) {
    let mut sound_pools = game_data.sound_pools.write()
        .expect("❌ Failed to acquire write lock on sound pools");

    let stream_handle = game_data.audio_stream_handle.read()
        .expect("❌ Failed to acquire read lock on audio stream")
        .clone();

    println!("🎵 Initializing Sound Pools...");

    if let Some(stream_handle) = stream_handle {
        for (sound_name, _, pool_size) in SOUND_FILES.iter() {
            println!("🔹 Initializing Sound Pool for '{}' with {} sinks", sound_name, pool_size);

            let mut pool = VecDeque::with_capacity(*pool_size as usize);
            for i in 0..*pool_size {
                match Sink::try_new(&stream_handle) {
                    Ok(sink) => {
                        let sink = Arc::new(sink);
                        pool.push_back(sink);
                        println!("   ✅ Created Sink {} for '{}'", i + 1, sound_name);
                    }
                    Err(e) => {
                        println!("   ❌ Failed to create Sink {} for '{}': {:?}", i + 1, sound_name, e);
                    }
                }
            }

            println!("   🎵 Final pool size for '{}': {}", sound_name, pool.len());
            sound_pools.insert(sound_name.to_string(), pool);
        }
    } else {
        println!("❌ Stream handle is None. Sound pools cannot be initialized!");
    }

    println!("✅ Sound Pools Initialized Successfully");
}
