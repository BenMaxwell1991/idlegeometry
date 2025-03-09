use crate::game::resources::bignumber::BigNumber;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub amount: BigNumber,
    pub rate: BigNumber,
    pub level: BigNumber,
    pub required: BigNumber,
    pub unlocked: bool,
}

impl Resource {
    pub fn new(name: &str, amount: BigNumber, rate: BigNumber, level: BigNumber, required: BigNumber, unlocked: bool) -> Self {
        let mut this = Self {
            name: name.to_string(),
            amount,
            rate,
            level,
            required,
            unlocked,
        };

        this.calculate_required();
        this
    }

    pub fn with_defaults(name: &str) -> Self {
        Self::new(name, BigNumber::new(0.0), BigNumber::new(0.0), BigNumber::new(0.0), BigNumber::new(0.0), false)
    }

    pub fn update(&mut self, delta_time: f64) {
        self.amount += self.rate * BigNumber::new(delta_time);
    }

    fn calculate_required(&mut self) {
        let base = 1.0;
        let level = self.level.to_f64();
        let required = base * 10.0_f64.powf(level);
        self.required = BigNumber::new(required);
    }
}

pub static DEFAULT_MOVE_SPEED: LazyLock<Resource> = LazyLock::new(|| {
    Resource::new(
        "movement_speed",
        BigNumber::new(400.0),
        BigNumber::new(0.0),
        BigNumber::new(0.0),
        BigNumber::new(0.0),
        true,
    )
});

pub static DEFAULT_HEALTH: LazyLock<Resource> = LazyLock::new(|| {
    Resource::new(
        "health",
        BigNumber::new(10.0),
        BigNumber::new(0.0),
        BigNumber::new(0.0),
        BigNumber::new(0.0),
        true,
    )
});

pub static DEFAULT_MANA: LazyLock<Resource> = LazyLock::new(|| {
    Resource::new(
        "mana",
        BigNumber::new(10.0),
        BigNumber::new(0.0),
        BigNumber::new(0.0),
        BigNumber::new(0.0),
        false,
    )
});