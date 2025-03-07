use crate::game::data::stored_data::StoredData;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use steamworks::Client;

#[derive(Clone)]
pub struct GameData {
    store: Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,
    steam_client: Arc<RwLock<Option<Client>>>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            steam_client: Arc::new(RwLock::new(None)),
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

    pub fn set_steam_client(&self, client: Client) {
        let mut steam_client_lock = self.steam_client.write().unwrap();
        *steam_client_lock = Some(client);
    }

    pub fn get_steam_client(&self) -> Option<Client> {
        self.steam_client.read().unwrap().clone()
    }
}