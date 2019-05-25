//player.rs
//
//
use quicksilver::{
    geom::Vector,
    graphics::Color,
};

//#[path = "item_bag.rs"] mod item_bag; // make mod.rs !!! ???
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
            pos: Vector::new(10,10), //rand or choose xxx
            ch: 'X', // xxx
            money: 50,
            energy: 80,
            name: "mike".to_string(),// make add
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
    pub fn remove_tool(&self, tool: &String) {
        //self.satchel.contains(tool)
    }
}