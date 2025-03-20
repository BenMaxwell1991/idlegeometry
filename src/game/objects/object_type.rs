use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum ObjectType {
    Player,
    Collectable,
    Enemy,
    Attack,
}