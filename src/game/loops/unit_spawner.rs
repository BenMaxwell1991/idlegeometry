use crate::game::data::game_data::GameData;
use crate::game::data::stored_data::GAME_MAP;
use crate::game::maths::pos_2::Pos2FixedPoint;
use crate::game::units::create_units::create_enemy_at_point;
use crate::game::units::unit::add_units;
use crate::ui::asset::sprite::sprite_sheet::YOUNG_RED_DRAGON;
use rand::random_range;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn spawn_units(game_data: Arc<GameData>) {
    if let Some(map) = game_data.get_field(GAME_MAP) {
    loop {
            let mut units = vec![];
            let mut positions = vec![];

            let map_x = map.width as i32 * map.tile_size;
            let map_y = map.height as i32 * map.tile_size;

            let pos = Pos2FixedPoint::new(random_range(0..=map_x), random_range(0..=map_y));
            units.push(create_enemy_at_point(YOUNG_RED_DRAGON));
            positions.push(pos);

            add_units(units, positions, &game_data);
            thread::sleep(Duration::from_micros(1_000_000));
        }
    }
}