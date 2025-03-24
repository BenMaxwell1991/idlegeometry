use crate::enums::gamestate::GameState;
use crate::enums::gamestate::GameState::Ready;
use crate::game::collision::spatial_hash_grid::SpatialHashGrid;
use crate::game::data::damage_numbers::DamageNumber;
use crate::game::data::stored_data::StoredData;
use crate::game::map::camera_state::CameraState;
use crate::game::map::game_map::GameMap;
use crate::game::maths::pos_2::Pos2FixedPoint;
use crate::game::objects::attacks::attack_stats::AttackName;
use crate::game::objects::game_object::GameObject;
use crate::helper::lock_helper::acquire_lock_mut;
use crate::ui::graphics::offscreen_renderer::OffscreenRenderer;
use device_query_revamped::Keycode;
use eframe::epaint::TextureHandle;
use egui::Vec2;
use glow::NativeProgram;
use rustc_hash::FxHashMap;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use steamworks::Client;

#[derive(Clone)]
pub struct GameData {
    pub game_loop_active: Arc<AtomicBool>,

    pub store: Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,
    pub steam_client: Arc<RwLock<Option<Client>>>,
    pub resources: Arc<RwLock<FxHashMap<String, f64>>>,
    pub game_map: Arc<RwLock<Option<GameMap>>>,

    pub units: Arc<RwLock<Vec<Option<GameObject>>>>,
    pub unit_positions: Arc<RwLock<Vec<Pos2FixedPoint>>>,
    pub empty_unit_indexes: Arc<RwLock<Vec<u32>>>,
    pub attack_pools: Arc<RwLock<FxHashMap<AttackName, Vec<GameObject>>>>,
    pub damage_numbers: Arc<RwLock<Vec<DamageNumber>>>,

    pub player_id: Arc<RwLock<Option<u32>>>,
    pub player_position: Arc<RwLock<Option<Pos2FixedPoint>>>,
    pub player_dead: Arc<RwLock<bool>>,

    pub spatial_hash_grid: Arc<RwLock<SpatialHashGrid>>,
    pub offscreen_renderer: Arc<RwLock<Option<OffscreenRenderer>>>,
    pub graphic_window_size: Arc<RwLock<Option<Vec2>>>,
    pub camera_state: Arc<RwLock<CameraState>>,
    pub rect_shader: Arc<RwLock<Option<NativeProgram>>>,
    pub sprite_shader: Arc<RwLock<Option<NativeProgram>>>,

    pub key_queue: Arc<RwLock<Vec<Keycode>>>,
    pub game_state: Arc<RwLock<GameState>>,
    pub icons: Arc<RwLock<FxHashMap<String, TextureHandle>>>,
    pub icons_inverted: Arc<RwLock<FxHashMap<String, TextureHandle>>>,
    pub current_track: Arc<RwLock<Option<String>>>,

    pub fonts: Arc<RwLock<FxHashMap<String, String>>>
}

impl GameData {
    pub fn new() -> Self {
        Self {
            game_loop_active: Arc::new(AtomicBool::new(false)),

            store: Arc::new(RwLock::new(HashMap::new())),
            steam_client: Arc::new(RwLock::new(None)),
            resources: Arc::new(RwLock::new(FxHashMap::default())),
            game_map: Arc::new(RwLock::new(None)),

            units: Arc::new(RwLock::new(Vec::new())),
            unit_positions: Arc::new(RwLock::new(Vec::new())),
            empty_unit_indexes: Arc::new(RwLock::new(Vec::new())),
            attack_pools: Arc::new(RwLock::new(FxHashMap::default())),
            damage_numbers: Arc::new(RwLock::new(Vec::new())),

            player_id: Arc::new(RwLock::new(None)),
            player_position: Arc::new(RwLock::new(None)),
            player_dead: Arc::new(RwLock::new(false)),

            spatial_hash_grid: Arc::new(RwLock::new(SpatialHashGrid::new())),
            offscreen_renderer: Arc::new(RwLock::new(None)),
            graphic_window_size: Arc::new(RwLock::new(None)),
            camera_state: Arc::new(RwLock::new(CameraState::new(Pos2FixedPoint::new(0,0), 2048))),
            rect_shader: Arc::new(RwLock::new(None)),
            sprite_shader: Arc::new(RwLock::new(None)),
            key_queue: Arc::new(RwLock::new(Vec::new())),
            game_state: Arc::new(RwLock::new(Ready)),
            icons: Arc::new(RwLock::new(FxHashMap::default())),
            icons_inverted: Arc::new(RwLock::new(FxHashMap::default())),

            current_track: Arc::new(RwLock::new(None)),

            fonts: Arc::new(RwLock::new(FxHashMap::default())),
        }
    }

    pub fn set_game_state(&self, game_state: GameState) {
        *acquire_lock_mut(&self.game_state, "game_state") = game_state;
        self.game_loop_active.store(game_state.is_game_active(), Ordering::Relaxed);
    }

    pub fn set_field<T: Any + Send + Sync>(&self, key: StoredData<T>, value: T) {
        let mut store = self.store.write().unwrap();
        store.insert(
            key.id.to_string(),
            Arc::new(RwLock::new(Box::new(value))),
        );
    }

    pub fn get_field<T: Any + Clone + Send + Sync>(&self, key: StoredData<T>) -> Option<T> {
        let store = self.store.read().unwrap();
        store.get(key.id).and_then(|value| {
            value.read().unwrap().downcast_ref::<T>().cloned()
        })
    }

    pub fn update_field<T: Any + Send + Sync>(&self, key: StoredData<T>, update_fn: impl FnOnce(&mut T)) {
        if let Some(value) = self.store.read().unwrap().get(key.id) {
            if let Ok(mut data) = value.write() {
                if let Some(data) = data.downcast_mut::<T>() {
                    update_fn(data);
                }
            }
        }
    }

    pub fn update_or_set<T: Any + Send + Sync>(
        &self,
        key: StoredData<T>,
        default_value: T,
        update_fn: impl FnOnce(&mut T),
    ) {
        if let Some(value) = self.store.read().unwrap().get(key.id) {
            if let Ok(mut data) = value.write() {
                if let Some(data) = data.downcast_mut::<T>() {
                    update_fn(data);
                    return;
                }
            }
        }

        let mut store = self.store.write().unwrap();
        store.insert(key.id.to_string(), Arc::new(RwLock::new(Box::new(default_value))));
    }
}