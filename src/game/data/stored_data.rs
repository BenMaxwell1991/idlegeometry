use crate::enums::gametab::GameTab;
use crate::game::loops::key_state::KeyState;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::resources::resource::Resource;
use crate::game::settings::Settings;
use crate::game::units::attack::Attack;
use crate::game::units::unit::Unit;
use crate::ui::asset::sprite::sprite_sheet::SpriteSheet;
use egui::Pos2;
use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use steamworks::Client;

pub struct StoredData<T: Any + Send + Sync> {
    pub id: &'static str,
    _marker: PhantomData<T>,
}

impl<T: Any + Send + Sync> StoredData<T> {
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

pub const CURRENT_TAB: StoredData<GameTab> = StoredData::new("current_tab");
pub const PLAYER_POSITION: StoredData<Pos2> = StoredData::new("player_position");
pub const SETTINGS: StoredData<Settings> = StoredData::new("settings");
pub const KEY_STATE: StoredData<Arc<KeyState>> = StoredData::new("key_state");
pub const RESOURCES: StoredData<Vec<Resource>> = StoredData::new("resources");
pub const STEAM_CLIENT: StoredData<Client> = StoredData::new("steam_client");
pub const GAME_MAP: StoredData<GameMap> = StoredData::new("game_map");
pub const CAMERA_STATE: StoredData<CameraState> = StoredData::new("camera_state");
pub const UNITS: StoredData<Vec<Unit>> = StoredData::new("units");
pub const ATTACKS: StoredData<Vec<Attack>> = StoredData::new("attacks");
pub const SPRITE_SHEETS: StoredData<HashMap<String, SpriteSheet>> = StoredData::new("sprite_sheets");
pub const GAME_IN_FOCUS: StoredData<bool> = StoredData::new("game_in_focus");