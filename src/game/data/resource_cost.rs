use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceAmount {
    pub food: Option<f64>,
    pub gold: Option<f64>,
    pub exp: Option<f64>,
    pub ruby: Option<f64>,
    pub gemstone: Option<f64>,
    pub experience: Option<f64>,
    pub fire: Option<f64>,
}

impl Default for ResourceAmount {
    fn default() -> Self {
        Self {
            food: None,
            gold: None,
            exp: None,
            ruby: None,
            gemstone: None,
            experience: None,
            fire: None,
        }
    }
}

impl ResourceAmount {
    pub fn default_provisions() -> Self {
        Self {
            food: Some(20.0),
            ..Self::default()
        }
    }

    pub fn can_afford(resources: &ResourceAmount, cost: &ResourceAmount) -> bool {
        macro_rules! afford_field {
            ($field:ident) => {
                cost.$field.map_or(true, |required| {
                    resources.$field.unwrap_or(0.0) >= required
                })
            };
        }

        afford_field!(gold)
            && afford_field!(ruby)
            && afford_field!(gemstone)
            && afford_field!(experience)
            && afford_field!(fire)
            && afford_field!(food)
    }

    pub fn pay_cost(resources: &mut ResourceAmount, cost: &ResourceAmount) {
        macro_rules! pay_field {
            ($field:ident) => {
                if let Some(amount) = cost.$field {
                    *resources.$field.get_or_insert(0.0) -= amount;
                }
            };
        }

        pay_field!(gold);
        pay_field!(ruby);
        pay_field!(gemstone);
        pay_field!(experience);
        pay_field!(fire);
        pay_field!(food);
    }

    pub fn add_production(resources: &mut ResourceAmount, production: &ResourceAmount) {
        macro_rules! add_field {
            ($field:ident) => {
                if let Some(amount) = production.$field {
                    *resources.$field.get_or_insert(0.0) += amount;
                }
            };
        }

        add_field!(gold);
        add_field!(ruby);
        add_field!(gemstone);
        add_field!(experience);
        add_field!(fire);
        add_field!(food);
    }

    pub fn provision_for_adventure(
        total_resources: &mut ResourceAmount,
        adventure_resources: &mut ResourceAmount,
        provisioning: &ResourceAmount,
    ) {
        *adventure_resources = ResourceAmount::default();
        macro_rules! provision_field {
            ($field:ident) => {
                if let Some(requested) = provisioning.$field {
                    let available = total_resources.$field.unwrap_or(0.0);
                    let to_provide = requested.min(available);

                    if to_provide > 0.0 {
                        *total_resources.$field.get_or_insert(0.0) -= to_provide;
                        *adventure_resources.$field.get_or_insert(0.0) += to_provide;
                    }
                }
            };
        }

        provision_field!(food);
        provision_field!(gold);
        provision_field!(ruby);
        provision_field!(gemstone);
        provision_field!(experience);
        provision_field!(fire);
    }
}