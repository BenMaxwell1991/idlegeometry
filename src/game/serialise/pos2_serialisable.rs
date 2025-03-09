use egui::Pos2;
use serde::{Deserialize, Serialize};

pub fn serialize_pos2<S>(pos: &Pos2, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    (pos.x, pos.y).serialize(serializer)
}

pub fn deserialize_pos2<'de, D>(deserializer: D) -> Result<Pos2, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let (x, y) = Deserialize::deserialize(deserializer)?;
    Ok(Pos2::new(x, y))
}