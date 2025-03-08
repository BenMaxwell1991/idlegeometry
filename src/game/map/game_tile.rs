use crate::game::map::tile_type::TileType;
use crate::game::map::tile_type::TileType::{Empty, SpawnPoint, Wall};

#[derive(Clone, Copy, PartialEq)]
pub struct GameTile {
    pub tile_type: TileType,
}

impl GameTile {
    pub const fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
        }
    }
}

pub const EMPTY_DEFAULT: GameTile = GameTile::new(Empty);
pub const WALL_DEFAULT: GameTile = GameTile::new(Wall);
pub const SPAWN_POINT_DEFAULT: GameTile = GameTile::new(SpawnPoint);