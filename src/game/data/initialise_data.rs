use crate::enums::gametab::GameTab;
use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::{CURRENT_TAB, KEY_STATE, SETTINGS};
use crate::game::loops::key_state::KeyState;
use crate::game::objects::attacks::attack_defaults::get_basic_attack;
use crate::game::objects::attacks::attack_stats::AttackName;
use crate::game::settings::Settings;
use std::sync::Arc;


pub fn initialise_data(game_data: GameData) -> GameData {

    // let (steam_client, single) = steamworks::Client::init_app(3585270).expect("Failed to initialize Steam");
    // println!("Logged in as: {}", steam_client.friends().name());
    // *acquire_lock_mut(&game_data.steam_client, "steam_client") = Some(steam_client);

    init_attacks(&game_data);
    println!("Initialised Attacks");

    game_data.set_field(KEY_STATE, Arc::new(KeyState::new()));
    game_data.set_field(CURRENT_TAB, GameTab::default());

    if game_data.get_field(SETTINGS).is_none() {
        println!("Initialised Settings");
        game_data.set_field(SETTINGS, Settings::default());
    }

    game_data
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