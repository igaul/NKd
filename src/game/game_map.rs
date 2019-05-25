//map.rs
//
//
use quicksilver::{
    geom::Vector,
    graphics::Color,
};
//derive for comp, vecs...
#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub pos: Vector,
    id: i32, //if useful for type
    pub ch: char, //for display during development
    chance_val: i32, //
    fare: i32, // cost to cross tile
    seen: bool, // tile seen by player
    pub color: Color, //replace with sprite
    //reqs: Bag, // required items to enter/traverse tile
    //...

}

// impl Tile {

// }

pub struct Map {
    pub map: Vec<Tile>, //???
    pub size: Vector,
    // x_size: i32,
    // y_size: i32,
}

impl Map {
    pub fn new(x: f32, y: f32) -> Map {
            Map{
                // y_size : y,
                // x_size : x,
                // size : y * x,
                size : Vector::new(x,y),
                map : Vec::with_capacity((x * y) as usize),
            }
    }
    pub fn gen(x: f32, y: f32) -> Map {
        let mut m = Map::new(x,y);
        for i in 0..x as i32 {
            for j in 0..y as i32{
                let mut t = Tile {
                    pos: Vector::new(i as f32, j as f32),
                    id: i + (j * x as i32),
                    ch: 'x',
                    chance_val: 1,
                    fare: 2,
                    seen: false,
                    color: Color::BLUE,

                };
                if i == 0 || i == x as i32 - 1 || j == 0 || j == y as i32 - 1 {
                t.ch = 'O';
            };
            m.map.push(t);
            }
        }
        m
    }
}

// impl Iterator for Map {
//     type item = Tile;
//     fn next(&mut self)-> Option<Tile> {

//     }
// }

