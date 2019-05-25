//map.rs
//map class
use super::Tile;

struct Map {
    map: Vec<Tile>,
    size: i32,
    x_size: i32,
    y_size: i32,
}

impl Map {
    pub fn new(x: i32, y: i32) -> Vec<Tile> {
        Map {
            y_size = y,
            x_size = x,
            size = y * x,
            map = Vec::with_capacity(size),
        }
    }
}

