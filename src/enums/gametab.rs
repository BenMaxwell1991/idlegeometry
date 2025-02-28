#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTab {
    Geometry,
    Upgrades,
    Settings,
    Shop,
}

impl Default for GameTab {
    fn default() -> Self {
        GameTab::Geometry
    }
}