//map.rs
//
//move into mod ???
use quicksilver::{geom::Vector, graphics::Color};
//derive for comp, vecs...
#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub pos: Vector,
    id: i32,         //if useful for type
    pub ch: char,    //for display during development
    chance_val: i32, //
    fare: i32,       // cost to cross tile
    seen: bool,      // tile seen by player
    pub color: Color, //replace with sprite
    pub reqs: Vec<String>, // required items to enter/traverse tile
                     //...
}

// impl Tile {

// }

pub struct Map {
    pub map: Vec<Tile>, //???
    pub size: Vector,
}

impl Map {
    pub fn new(x: f32, y: f32) -> Map {
        Map {
            size: Vector::new(x, y),
            map: Vec::with_capacity((x * y) as usize),
        }
    }
    pub fn gen(x: f32, y: f32) -> Map {
        let mut m = Map::new(x, y);
        for i in 0..x as i32 {
            for j in 0..y as i32 {
                let mut t = Tile {
                    pos: Vector::new(i as f32, j as f32),
                    id: i + (j * x as i32),
                    ch: 'x',
                    chance_val: 1,
                    fare: 2,
                    seen: false,
                    color: Color::BLUE,
                    reqs: ["Blue towel".to_string()].to_vec(),
                };
                if i == 0 || i == x as i32 - 1 || j == 0 || j == y as i32 - 1 {
                    t.ch = 'O';
                };
                m.map.push(t);
            }
        }
        m
    }

    pub fn is_on_board(&self, o_pos: Vector) -> bool {
        (o_pos.x >= 0.0 && o_pos.x <= self.size.x) &&
        (o_pos.y >= 0.0 && o_pos.y <= self.size.y)
    }

}
#[test]
fn test_is_on_board(){
    let m = Map::new(10.0,10.0);

    let p1 = m.is_on_board(Vector::new(0.0,0.0));
    let p2 = m.is_on_board(Vector::new(10.0,10.0));
    let p3 = m.is_on_board(Vector::new(5.0,5.0));

    let pf1 = m.is_on_board(Vector::new(-1.0,-1.0));
    let pf2 = m.is_on_board(Vector::new(11.0,10.1));
    let pf3 = m.is_on_board(Vector::new(5.0,15.0));

    assert_eq!((p1, p2, p3), (true, true, true));
    assert_eq!((pf1,pf2,pf3),(false, false, false));
}

// impl Iterator for Map {
//     type item = Tile;
//     fn next(&mut self)-> Option<Tile> {

//     }
// }
