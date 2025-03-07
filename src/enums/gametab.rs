use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameTab {
    Geometry,
    Upgrades,
    Settings,
    Shop,
    NullGameTab,
}

impl Default for GameTab {
    fn default() -> Self {
        GameTab::Geometry
    }
}