//player.rs
//
//
use quicksilver::{geom::Vector, graphics::Color};

pub struct Player {
    pub pos: Vector,
    pub ch: char, // xxx
    pub money: i32,
    pub energy: i32,
    pub name: String,
    pub satchel: super::item_bag::ItemBag,
    pub color: Color,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector::new(10, 10), //rand or choose xxx
            ch: 'X',                  // xxx
            money: 50,
            energy: 80,
            name: "mike".to_string(), // make add
            satchel: super::item_bag::ItemBag::new(),
            color: Color::RED,
        }
    }
    pub fn has_tool(&self, tool: &String) -> bool {
        self.satchel.contains(tool)
    }
    pub fn add_tool(&mut self, tool: &String) {
        self.satchel.add(tool)
    }
    pub fn remove_tool(&mut self, tool: &String) {
        self.satchel.remove(tool)
    }
    pub fn contents_to_string(&self) -> String {
        // let mut s = "".to_string();
        // for (k,v) in self.satchel.contents() {

        // }
        self.satchel.contents_as_strings().join("\n")
    }
    pub fn can_move(&self, next: &super::game_map::Tile) -> bool {
        if self.satchel.compare_to_tile_reqs(&next.reqs) == "" {
            return true
        }
        false
    }
}
