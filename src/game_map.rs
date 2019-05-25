//map.rs
//
//
use quicksilver::*;

//#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pos: quicksilver::geom::Vector,
    id: i32, //if useful for type
    ch: char, //for display during development
    chance_val: i32, //
    fare: i32, // cost to cross tile
    seen: bool, // tile seen by player
    //reqs: Bag, // required items to enter/traverse tile
    //...

}

// impl Tile {

// }

pub struct Map {
    pub map: Vec<Tile>,
    size: i32,
    x_size: i32,
    y_size: i32,
}

impl Map {
    pub fn new(x: i32, y: i32) -> Map {
            Map{
                y_size : y,
                x_size : x,
                size : y * x,
                map : Vec::with_capacity((x * y) as usize),
            }
    }
    pub fn gen(x: i32, y: i32) -> Map {
        let mut m = Map::new(x,y);
        for i in 0..x {
            for j in 0..y {
                let mut t = Tile {
                    pos: geom::Vector::new(i as f32, j as f32),
                    id: i + (j * x),
                    ch: 'x',
                    chance_val: 1,
                    fare: 2,
                    seen: false,

                };
                if i == 0 || i == x - 1 || j == 0 || j == y - 1 {
                t.ch = 'O';
            };
            m.map.push(t);
            }
        }
        m
    }
}

