use crate::game::collision::spatial_hash_grid::SpatialHashGrid;
use crate::game::data::stored_data::StoredData;
use crate::game::units::unit::Unit;
use egui::Vec2;
use glow::Context;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use eframe::emath::Pos2;

#[derive(Clone)]
pub struct GameData {
    pub store: Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,
    pub units: Arc<RwLock<Vec<Option<Unit>>>>,
    pub unit_positions: Arc<RwLock<Vec<Pos2>>>,
    pub empty_unit_indexes: Arc<RwLock<Vec<u32>>>,
    pub spatial_hash_grid: Arc<RwLock<SpatialHashGrid>>,
    pub gl_context: Arc<RwLock<Option<Arc<Context>>>>,
    pub graphic_window_size: Arc<RwLock<Option<Vec2>>>
}

impl GameData {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            units: Arc::new(RwLock::new(Vec::new())),
            unit_positions: Arc::new(RwLock::new(Vec::new())),
            empty_unit_indexes: Arc::new(RwLock::new(Vec::new())),
            spatial_hash_grid: Arc::new(RwLock::new(SpatialHashGrid::new())),
            gl_context: Arc::new(RwLock::new(None)),
            graphic_window_size: Arc::new(RwLock::new(None)),
        }
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