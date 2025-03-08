use crate::enums::gametab::GameTab;
use crate::game::key_state::KeyState;
use crate::game::settings::Settings;
use crate::resources::resource::Resource;
use egui::Pos2;
use std::any::Any;
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