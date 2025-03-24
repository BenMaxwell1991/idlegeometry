use crate::enums::gametab::GameTab;
use crate::game::loops::key_state::KeyState;
use crate::game::resources::resource::Resource;
use crate::game::settings::Settings;
use crate::ui::asset::sprite::sprite_sheet::SpriteSheet;
use rustc_hash::FxHashMap;
use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;

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
pub const SETTINGS: StoredData<Settings> = StoredData::new("settings");
pub const KEY_STATE: StoredData<Arc<KeyState>> = StoredData::new("key_state");
pub const RESOURCES: StoredData<Vec<Resource>> = StoredData::new("resources");
pub const SPRITE_SHEETS_NATIVE: StoredData<FxHashMap<String, SpriteSheet>> = StoredData::new("sprite_sheets_native");
pub const GAME_IN_FOCUS: StoredData<bool> = StoredData::new("game_in_focus");