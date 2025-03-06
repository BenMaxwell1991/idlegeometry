use crate::game::game_data::GameData;
use crate::game::settings::Settings;
use crate::resources::bignumber::BigNumber;
use crate::resources::resource::Resource;

pub fn init(game_data: GameData) -> GameData {

    if game_data.get_field::<Vec<Resource>>("resources").is_none() {
        println!("No saved game found, starting a new game.");
        game_data.set_field("resources", vec![
            Resource::new("Points", BigNumber::new(0.0), BigNumber::new(0.03), BigNumber::new(0.0), BigNumber::new(0.0), true),
            Resource::with_defaults("Lines"),
            Resource::with_defaults("Triangles"),
        ]);
    }

    if game_data.get_field::<Settings>("settings").is_none() {
        game_data.set_field("settings", Settings::default());
    }

    game_data
}