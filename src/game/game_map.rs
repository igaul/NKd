//map.rs
//
//move into mod ???
use quicksilver::{geom::Vector, graphics::Color};
//derive for comp, vecs...
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Tile {
    pub pos: Vector,
    pub id: i32,         //if useful for type
    pub ch: char,    //for display during development
    pub chance_val: i32, //
    pub fare: i32,       // cost to cross tile
    pub seen: bool,      // tile seen by player
    pub color: Color, //replace with sprite
    pub reqs: Vec<String>, // required items to enter/traverse tile
                     //...
}
// impl Default for Tile {
//     fn default() -> Tile {
//         pos: quicksilver::geom::Vector::new(0.0 as f32, 0.0 as f32),
//         id: 0,
//         ch: 'z'
//         chance_val: 0,
//         fare: 99,
//         seen: false,
//         color: Color::BLACK,
//         reqs: Vec<String>::with_capacity(0),
//     }
// }
impl Tile {
    pub fn new(pos_x: i32, pos_y: i32, id: i32, tile_type: char) -> Tile{
        match tile_type {
            'x' => Tile {
                    pos: Vector::new(pos_x as f32, pos_y as f32),
                    id: id,
                    ch: tile_type,
                    chance_val: 1,
                    fare: 2,
                    color: Color::BLUE,
                    reqs: ["Blue towel".to_string()].to_vec(),
                    ..Default::default()
                },
            'm' => Tile {
                    pos: Vector::new(pos_x as f32, pos_y as f32),
                    id: id,
                    ch: tile_type,
                    chance_val: 1,
                    fare: 10,
                    color: Color::from_hex("#A52A2A"),
                    reqs: ["Rope".to_string()].to_vec(),
                    ..Default::default()
                },
            _ => Tile { pos: Vector::new(pos_x as f32, pos_y as f32), id: id, ch: tile_type, ..Default::default() }
        }
    }
    pub fn set_seen(&mut self, seen: bool) {
        self.seen = seen;
    }
}

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
    pub fn gen(x: i32, y: i32) -> Map {
        let mut m = Map::new(x as f32, y as f32);
        for i in 0..x {
            for j in 0..y {
                let mut t: Tile;
                if (i + j) % 11 == 2 {// xxx make rand
                    t = Tile::new(i, j ,i + j * x, 'm');
                } 
                else {t = Tile::new(i, j ,i + j * x, 'x');}                
                //{
                //     pos: Vector::new(i as f32, j as f32),
                //     id: i + (j * x),
                //     ch: 'x',
                //     chance_val: 1,
                //     fare: 2,
                //     seen: false,
                //     color: Color::BLUE,
                //     reqs: ["Blue towel".to_string()].to_vec(),
                // };
                if i == 0 || i == x  - 1 {
                    t.ch = 'O';
                    t.color = Color::GREEN;
                    t.reqs.push("Green towel".to_string());
                };
                if j == 0 || j == y - 1 {
                    t.ch = 'O';
                    t.color = Color::ORANGE;
                    t.reqs.push("Orange towel".to_string());
                }
                m.map.push(t);
            }
        }
        m
    }
    pub fn get_tile(&self, pos: &Vector) -> &Tile { // result ???
        let mut i = 0.0; //make default reqs xxx ???
        if self.is_on_board(*pos){
            i = pos.y + pos.x * self.size.x; //must be usizable
        }      
        &self.map[i as usize]
    }
    pub fn get_mut_tile(&mut self, pos: Vector) -> &mut Tile {
        let mut i = 0.0;
        if self.is_on_board(pos) {
            i = pos.y + pos.x * self.size.x;
        }
        &mut self.map[i as usize]
    }

    pub fn pos_to_tile_id(pos: &Vector, width: f32) -> usize {
        (pos.y + pos.x * width) as usize
    }

    pub fn is_on_board(&self, o_pos: Vector) -> bool { // make into vector trait ???
        (o_pos.x >= 0.0 && o_pos.x <= self.size.x - 1.0) &&
        (o_pos.y >= 0.0 && o_pos.y <= self.size.y - 1.0)
    }
    pub fn is_on_board_x(&self, other: f32) -> bool { //<T: Into<f32>>
        (other >= 0.0 && other <= self.size.x - 1.0)
    }
    pub fn is_on_board_y(&self, other: f32) -> bool {
        (other >= 0.0 && other <= self.size.y - 1.0)
    }

}
#[test]
fn test_is_on_board(){
    let m = Map::new(10.0,10.0);

    let p1 = m.is_on_board(Vector::new(0.0,0.0));
    let p2 = m.is_on_board(Vector::new(9.0,9.0));
    let p3 = m.is_on_board(Vector::new(5.0,5.0));

    let pf1 = m.is_on_board(Vector::new(-1.0,-1.0));
    let pf2 = m.is_on_board(Vector::new(11.0,10.1));
    let pf3 = m.is_on_board(Vector::new(5.0,15.0));

    assert_eq!((p1, p2, p3), (true, true, true));
    assert_eq!((pf1,pf2,pf3),(false, false, false));
}
#[test]
fn test_is_on_board_x_y(){
    let m = Map::new(10.0,10.0);

    let p1 = m.is_on_board_x(0.0);
    let p2 = m.is_on_board_x(9.0);
    let p3 = m.is_on_board_x(5.0);

    let pf1 = m.is_on_board_x(-1.0);
    let pf2 = m.is_on_board_x(10.0);
    let pf3 = m.is_on_board_x(100.1);

    assert_eq!((p1, p2, p3), (true, true, true));
    assert_eq!((pf1,pf2,pf3),(false, false, false));

    let p1 = m.is_on_board_y(0.0);
    let p2 = m.is_on_board_y(9.0);
    let p3 = m.is_on_board_y(5.0);

    let pf1 = m.is_on_board_y(-1.0);
    let pf2 = m.is_on_board_y(10.0);
    let pf3 = m.is_on_board_y(100.1);

    assert_eq!((p1, p2, p3), (true, true, true));
    assert_eq!((pf1,pf2,pf3),(false, false, false));
}
//test map gen
// impl Iterator for Map {
//     type item = Tile;
//     fn next(&mut self)-> Option<Tile> {

//     }
// }
