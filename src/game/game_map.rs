//map.rs
//
//
use rand::random as rand;
use quicksilver::{
    geom::Vector,
    graphics::Color,
};
//derive for comp, vecs...
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Tile {
    pub pos: Vector,
    pub id: i32,         //if useful for type
    pub ch: char,        //for display during development
    pub chance_val: i32, //
    pub fare: i32,       // cost to cross tile
    pub seen: bool,      // tile seen by player
    pub color: Color,    //replace with sprite ???
    pub reqs: Vec<String>, // required items to enter/traverse tile
    

    //...
}

impl Tile {
    pub fn new(pos: Vector, id: i32) -> Tile {
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
    pub fn get_display_ch(&self) -> &char {
        if self.seen {
            return &self.ch;
        }
        &'x'
    }
    pub fn get_display_color(&self) -> Color {
        if self.seen {
            return self.color;
        }
        Color::BLACK
    }
    pub fn mod_tile(
        &mut self,
        tile_type: char,
        chance_val: i32,
        fare: i32,
        color: Color,
        reqs: Vec<String>,
    ) {
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
                self.chance_val = rand::<u8>() as i32 % 10; //keep range < 256
                self.fare = rand::<u8>() as i32 % 10;
                self.color = Color::from_hex("#006400"); //dark green
                self.reqs.push("Blue towel".to_string());
            }
            'l' => {
                self.color = Color::GREEN;
                self.reqs.push("Green towel".to_string());
            }
            'm' => {
                self.fare = 10;
                self.color = Color::from_hex("#A52A2A"); //brown ish
                self.reqs.push("Rope".to_string())
            }
            'w' => {
                self.color = Color::BLUE;
                self.reqs.push("Boat".to_string());
            }
            'o' => {
                self.color = Color::ORANGE;
                self.reqs.push("Orange towel".to_string());
            }
            'g' => {
                println!("{}", rand::<u8>());
                println!("{}", rand::<u8>() % 10);
                println!("{}", rand::<u8>() / 100);
                self.fare = 10;
                self.color = Color::from_hex("#FFD700"); //gold ish
            }
            _ => {}
        };
    }

    pub fn set_seen(&mut self, seen: bool) {
        self.seen = seen;
    }
}

pub struct Map {
    pub map: Vec<Tile>, //???
    pub size: Vector,
    pub win: bool,
}

// make default/shrouded display char/color xxx
impl Map {
    pub fn new(x: i32, y: i32) -> Map {
        Map {
            size: Vector::new(x, y),
            map: Vec::with_capacity((x * y) as usize),
            win: false,
        }
    }
    pub fn gen(x: i32, y: i32) -> Map {
        let mut m = Map::new(x, y);
        for i in 0..x {
            for j in 0..y {
                let pos = Vector::new(i, j);
                let id = i + j * x;
                let mut t = Tile::new(pos, id);
                let r = rand::<u8>() % 10;
                if i == 0 || i == x - 1 {
                    t.auto_mod_tile('l');
                } else if j == 0 || j == y - 1 {
                    t.auto_mod_tile('o');
                } else if r % 11 == 2 {
                    t.auto_mod_tile('w');
                } else if r > 100 {
                    t.auto_mod_tile('m')
                } else if r == 80 {
                    t.auto_mod_tile('g')
                } else {
                    t.auto_mod_tile('x');
                }

                m.map.push(t);
            }
        }
        m
    }
    pub fn get_tile(&self, pos: Vector) -> &Tile {
        if self.is_on_board(pos) {
            //if valid vector position, return tile from map's vec of tiles
            return &self.map[(pos.y + pos.x * self.size.x) as usize] //must be usizable
        } //else return tile 0 as default
        return &self.map[0]
    }
    pub fn get_mut_tile(&mut self, pos: Vector) -> Option<&mut Tile> {
        if self.is_on_board(pos) {
            Some(&mut self.map[(pos.y + pos.x * self.size.x) as usize])
        } 
        else {//return none
        None }
    }

    pub fn pos_to_tile_id(pos: Vector, width: f32) -> usize {
        (pos.y + pos.x * width) as usize
    }

    pub fn is_on_board(&self, o_pos: Vector) -> bool {
        // make into vector trait ???
        (o_pos.x >= 0.0 && o_pos.x <= self.size.x - 1.0)
            && (o_pos.y >= 0.0 && o_pos.y <= self.size.y - 1.0)
    }
    pub fn is_on_board_x(&self, other: f32) -> bool {
        //<T: Into<f32>>
        (other >= 0.0 && other <= self.size.x - 1.0)
    }
    pub fn is_on_board_y(&self, other: f32) -> bool {
        (other >= 0.0 && other <= self.size.y - 1.0)
    }
    //tiles to be unshrouded
    pub fn unshroud_dis_x(&mut self, pos: Vector, dis: i32) {
        for x in 0..=dis * 2 {
            //offset range nonneg
            for y in 0..=dis * 2 {
                let offset = Vector::new(x as i32 - dis, y as i32 - dis); // xxx
                match self.get_mut_tile(pos + offset) {
                    Some(t) => t.set_seen(true),
                    None => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_board() {
        let m = Map::new(10, 10);

        let p1 = m.is_on_board(Vector::new(0.0, 0.0));
        let p2 = m.is_on_board(Vector::new(9, 9));
        let p3 = m.is_on_board(Vector::new(5.0, 5.0));

        let pf1 = m.is_on_board(Vector::new(-1.0, -1.0));
        let pf2 = m.is_on_board(Vector::new(9.0, 9.1));
        let pf3 = m.is_on_board(Vector::new(5.0, 15.0));

        assert_eq!(p1 && p2 && p3, true);
        assert_eq!(pf1 && pf2 && pf3, false);;
    }
    #[test]
    fn test_is_on_board_x_y() {
        let m = Map::new(10, 10);

        let p1 = m.is_on_board_x(0.0);
        let p2 = m.is_on_board_x(9.0);
        let p3 = m.is_on_board_x(5.0);

        let pf1 = m.is_on_board_x(-1.0);
        let pf2 = m.is_on_board_x(10.0);
        let pf3 = m.is_on_board_x(100.1);

        assert_eq!(p1 && p2 && p3, true);
        assert_eq!(pf1 && pf2 && pf3, false);

        let p1 = m.is_on_board_y(0.0);
        let p2 = m.is_on_board_y(9.0);
        let p3 = m.is_on_board_y(5.0);

        let pf1 = m.is_on_board_y(-1.0);
        let pf2 = m.is_on_board_y(10.0);
        let pf3 = m.is_on_board_y(100.1);

        assert_eq!(p1 && p2 && p3, true);
        assert_eq!(pf1 && pf2 && pf3, false);
    }

    #[test]
    fn test_shroud() {
        let mut m = Map::gen(10, 10);
        //four corners
        let (p1, p2, p3, p4) = (
            Vector::new(0, 0),
            Vector::new(9, 0),
            Vector::new(0, 9),
            Vector::new(9, 9),
        );
        //3 away
        let (p1a, p2a, p3a, p4a) = (
            Vector::new(3, 0),
            Vector::new(9, 3),
            Vector::new(3, 9),
            Vector::new(6, 6),
        );

        assert_eq!(
            m.get_tile(p1).seen
                && m.get_tile(p2).seen
                && m.get_tile(p3).seen
                && m.get_tile(p4).seen,
            false
        );
        assert_eq!(
            m.get_tile(p1a).seen
                && m.get_tile(p2a).seen
                && m.get_tile(p3a).seen
                && m.get_tile(p4a).seen,
            false
        );
        //unshroud four corners
        m.unshroud_dis_x(p1, 2);
        m.unshroud_dis_x(p2, 2);
        m.unshroud_dis_x(p3, 2);
        m.unshroud_dis_x(p4, 2);

        assert_eq!(
            m.get_tile(p1).seen
                && m.get_tile(p2).seen
                && m.get_tile(p3).seen
                && m.get_tile(p4).seen,
            true
        ); //points are seen
        assert_eq!(
            m.get_tile(p1a).seen
                && m.get_tile(p2a).seen
                && m.get_tile(p3a).seen
                && m.get_tile(p4a).seen,
            false
        ); //outside of range are not
    }
    //test map gen
    //
}
