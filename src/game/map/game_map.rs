use crate::game::map::game_tile::{GameTile, EMPTY_DEFAULT, SPAWN_POINT_DEFAULT, WALL_DEFAULT};
use std::collections::HashMap;

#[derive(Clone)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: HashMap<(usize, usize), GameTile>,
    pub tile_size: i32,
}

impl GameMap {
    pub fn new(width: usize, height: usize, tile_size: i32) -> Self {
        let mut tiles = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    tiles.insert((x, y), WALL_DEFAULT);
                } else {
                    tiles.insert((x, y), SPAWN_POINT_DEFAULT);
                }
            }
        }

        Self { width, height, tiles, tile_size }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> GameTile {
        *self.tiles.get(&(x, y)).unwrap_or(&EMPTY_DEFAULT)
    }
}
