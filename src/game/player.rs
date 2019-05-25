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
    pub satchel: super::item_bag::Item_Bag,
    pub color: Color,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector::new(1,1), //rand or choose xxx
             ch: 'A', // xxx
            money: 111,
            energy: 222,
            name: "mike".to_string(),// make add
            satchel: super::item_bag::Item_Bag::new(),
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