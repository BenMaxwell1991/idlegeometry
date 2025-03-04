use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct GameData {
    store: Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set_field<T: Any + Send + Sync>(&self, key: &str, value: T) {
        let mut store = self.store.write().unwrap();
        store.insert(
            key.to_string(),
            Arc::new(RwLock::new(Box::new(value))),
        );
    }

    pub fn get_field<T: Any + Clone>(&self, key: &str) -> Option<T> {
        let store = self.store.read().unwrap();
        store.get(key).and_then(|value| {
            value.read().unwrap().downcast_ref::<T>().cloned()
        })
    }

    pub fn update_field<T: Any + Clone>(&self, key: &str, update_fn: impl FnOnce(&mut T)) {
        if let Some(value) = self.store.read().unwrap().get(key) {
            if let Ok(mut data) = value.write() {
                if let Some(data) = data.downcast_mut::<T>() {
                    update_fn(data);
                }
            }
        }
    }
}
