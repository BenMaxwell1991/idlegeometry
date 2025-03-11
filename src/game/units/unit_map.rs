use crate::game::units::unit::Unit;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct UnitMap {
    pub(crate) map: HashMap<Uuid, usize>, // Maps UUID → index in Vec<Unit>
}

impl UnitMap {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn set_reserve(&mut self, size: usize) {
        self.map.reserve(size);
    }

    pub fn add_unit(&mut self, unit: &Unit, index: usize) {
        self.map.insert(unit.id, index);
    }

    pub fn get_unit_index(&self, id: &Uuid) -> Option<usize> {
        self.map.get(id).copied()
    }
}
