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
//         pos: quicksilver::geom::Vector::new(0, 0),
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
    pub fn new(pos: Vector, id: i32) -> Tile{
        Tile {
            pos,
            id,
            ch: 'x',
            chance_val: 0,
            fare: 99,
            seen: false,
            color: Color::BLACK,
            reqs: Vec::<String>::with_capacity(0),
        }
    }
    pub fn mod_tile(&mut self, tile_type: char, chance_val: i32, fare: i32, color: Color, reqs:Vec<String>) {
        self.ch = tile_type;
        self.chance_val = chance_val;
        self.fare = fare;
        self.color = color;
        self.reqs = reqs;
    }
    pub fn auto_mod_tile(&mut self, tile_type: char) {
         self.ch = tile_type;
         match tile_type {
            'x' => {
                self.chance_val = 1;
                self.fare = 2;
                self.color = Color::from_hex("#006400");//dark green
                self.reqs.push("Blue towel".to_string());
            },
            'l' => {
                self.color = Color::GREEN;
                self.reqs.push("Green towel".to_string());
            },
            'm' => {
                self.fare = 10;
                self.color = Color::from_hex("#A52A2A");//brown ish
                self.reqs.push("Rope".to_string())
            },
            'o' => {
                self.color = Color::ORANGE;
                self.reqs.push("Orange towel".to_string());
            },
            _ => {},
        };
}


        // self.ch = tile_type;
        // self.chance_val = chance_val;
        // self.fare = fare;
        // self.color = color;
        // self.reqs = reqs;
    
        // match tile_type {
        //     'x' => {
        //         t.chance_val = 1;
        //         t.fare = 2;
        //         t.color = Color::from_hex("#006400");//dark green
        //         t.reqs.push("Blue towel".to_string());
        //     },
        //     'O' => {
        //         t.color = Color::GREEN;
        //         t.reqs.push("Green towel".to_string());
        //     },
        //     'M' => {
        //         t.fare = 10;
        //         t.color = Color::from_hex("#A52A2A");//brown ish
        //         //t.reqs.push("Rope".to_string())
        //     },
        //     'o' => {
        //         t.color = Color::ORANGE;
        //         t.reqs.push("Orange towel".to_string());
        //     },
        //     _ => {},
        // };
        // t
        // match tile_type {
        //     'x' => Tile {
        //             pos: Vector::new(pos_x, pos_y),
        //             id: id,
        //             ch: tile_type,
        //             chance_val: 1,
        //             fare: 2,
        //             color: Color::BLUE,
        //             reqs: ["Blue towel".to_string()].to_vec(),
        //             ..Default::default()
        //         },
        //     'm' => Tile {
        //             pos: Vector::new(pos_x, pos_y),
        //             id: id,
        //             ch: tile_type,
        //             chance_val: 1,
        //             fare: 10,
        //             color: Color::from_hex("#A52A2A"),//brown ish
        //             reqs: ["Rope".to_string()].to_vec(),
        //             ..Default::default()
        //         },
        //     // 'O' => {
        //     //     t.ch = 'O';
        //     //         t.color = Color::GREEN;
        //     //         t.reqs.push("Green towel".to_string());
        //     // },
        //     _ => Tile { pos: Vector::new(pos_x, pos_y), id: id, ch: tile_type, ..Default::default() }
        // }
    //}
    pub fn set_seen(&mut self, seen: bool) {
        self.seen = seen;
    }
}

pub struct Map {
    pub map: Vec<Tile>, //???
    pub size: Vector,
}

impl Map {
    pub fn new(x: i32, y: i32) -> Map {
        Map {
            size: Vector::new(x, y),
            map: Vec::with_capacity((x * y) as usize),
        }
    }
    pub fn gen(x: i32, y: i32) -> Map {
        let mut m = Map::new(x, y);
        for i in 0..x {
            for j in 0..y {
                let pos = Vector::new(i,j);
                let id = i + j * x;
                let mut t = Tile::new(pos,id);

                if i == 0 || i == x  - 1 {
                    t.auto_mod_tile('l');
                }
                else if j == 0 || j == y - 1 {
                    t.auto_mod_tile('o');
                }
                else if (i * j) % 11 == 2 {// xxx make rand
                    t.auto_mod_tile('m');
                } 
                else {t.auto_mod_tile('x');}                
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
                // if i == 0 || i == x  - 1 {
                //     t.ch = 'O';
                //     t.color = Color::GREEN;
                //     t.reqs.push("Green towel".to_string());
                // };
                // if j == 0 || j == y - 1 {
                //     t.ch = 'O';
                //     t.color = Color::ORANGE;
                //     t.reqs.push("Orange towel".to_string());
                // }
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
    let m = Map::new(10,10);

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
