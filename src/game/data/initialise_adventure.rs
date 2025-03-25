use std::sync::atomic::Ordering;
use std::time::Duration;
use rand::random_range;
use crate::game::data::game_data::GameData;
use crate::game::data::resource_cost::ResourceAmount;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::maths::pos_2::{Pos2FixedPoint, FIXED_POINT_SCALE};
use crate::game::objects::animation::Animation;
use crate::game::objects::attacks::attack_defaults::get_modified_attack;
use crate::game::objects::attacks::attack_stats::AttackName;
use crate::game::objects::game_object::{add_units, GameObject};
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use crate::game::objects::unit_defaults::{create_01_baby_dragon, create_02_aqua_drake, create_03_adult_white_dragon};
use crate::game::objects::upgrades::{Upgrade, UpgradeType};
use crate::game::resources::resource::DEFAULT_MOVE_SPEED;
use crate::helper::lock_helper::{acquire_lock, acquire_lock_mut};
use crate::ui::asset::sprite::sprite_sheet::BABY_GREEN_DRAGON;

const TILE_SIZE: i32 = 40 * FIXED_POINT_SCALE;
const X_TILE_COUNT: usize = 60;
const Y_TILE_COUNT: usize = 60;
const X_CENTER: i32 = TILE_SIZE * X_TILE_COUNT as i32 / 2;
const Y_CENTER: i32 = TILE_SIZE * Y_TILE_COUNT as i32 / 2;

pub fn initialise_adventure(game_data: &GameData) {
    init_map(game_data);
    println!("Adventure Map Initialized");

    init_player(game_data);
    println!("Adventure Player Initialized");

    init_enemies(game_data);
    println!("Adventure Enemies Initialized");

    init_resources(game_data);
    println!("Adventure Resources Initialized");

    init_reset(game_data);
    println!("Reset complete flagged as false");;
}

fn init_map(game_data: &GameData) {
    *acquire_lock_mut(&game_data.game_map, "game_map") = Some(GameMap::new(X_TILE_COUNT, Y_TILE_COUNT, TILE_SIZE));
    *acquire_lock_mut(&game_data.camera_state, "camera_state") = CameraState::new(Pos2FixedPoint::new(X_CENTER, Y_CENTER), 2048);
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

    *acquire_lock_mut(&game_data.player_id, "player_dead") = player_id;
    *acquire_lock_mut(&game_data.player_position, "player_dead") = Some(player_position);
    *acquire_lock_mut(&game_data.player_dead, "player_dead") = false;
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

fn init_resources(game_data: &GameData) {
    let mut player_data = acquire_lock_mut(&game_data.player_data, "player_data");
    let mut adventure_resources = acquire_lock_mut(&game_data.resource_amounts, "resources");
    ResourceAmount::provision_for_adventure(&mut player_data.resources_persistent, &mut adventure_resources, &ResourceAmount::default_provisions());
}

fn init_reset(game_data: &GameData) {
    game_data.reset_complete.store(false, Ordering::Relaxed);
}