use crate::resources::bignumber::BigNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub amount: BigNumber,
    pub rate: BigNumber,
    pub unlocked: bool,
}

impl Resource {
    pub fn new(name: &str, amount: BigNumber, rate: BigNumber, unlocked: bool) -> Self {
        Self {
            name: name.to_string(),
            amount,
            rate,
            unlocked,
        }
    }

    pub fn with_defaults(name: &str) -> Self {
        Self::new(name, BigNumber::new(0.0), BigNumber::new(0.0), false)
    }

    pub fn update(&mut self, delta_time: f64) {
        self.amount += self.rate * BigNumber::new(delta_time);
    }
}