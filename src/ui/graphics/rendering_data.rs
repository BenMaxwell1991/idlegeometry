use crate::game::data::damage_numbers::DamageNumber;
use crate::game::data::game_data::GameData;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::maths::pos_2::Pos2FixedPoint;
use crate::game::objects::game_object::GameObject;
use crate::helper::lock_helper::acquire_lock;
use egui::Vec2;
use std::sync::Arc;

pub struct RenderData {
    pub game_units: Vec<Option<GameObject>>,
    pub unit_positions: Vec<Pos2FixedPoint>,
    pub camera_state: CameraState,
    pub damage_numbers: Vec<DamageNumber>,
    pub game_map: Option<GameMap>,
    pub window_size: Option<Vec2>,
}

impl RenderData {
    pub fn from(game_data: Arc<GameData>) -> Self {
        let game_units = acquire_lock(&game_data.units, "game_units").clone();
        let unit_positions = acquire_lock(&game_data.unit_positions, "unit_positions").clone();
        let camera_state = acquire_lock(&game_data.camera_state, "camera_state").clone();
        let damage_numbers = acquire_lock(&game_data.damage_numbers, "damage_numbers").clone();
        let game_map = acquire_lock(&game_data.game_map, "game_map").clone();
        let window_size = acquire_lock(&game_data.graphic_window_size, "window_size").clone();

        Self {
            game_units,
            unit_positions,
            camera_state,
            damage_numbers,
            game_map,
            window_size,
        }


    }
}