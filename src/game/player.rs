//player.rs
//
//
use quicksilver::{geom::Vector, graphics::Color};
use super::item_bag::ItemBag as Bag;
pub struct Player {
    pub pos: Vector,
    pub ch: char, // xxx
    pub money: i32,
    pub energy: i32,
    pub name: String,
    pub satchel: Bag,
    pub color: Color,
    pub act: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector::new(10, 10), //rand or choose xxx
            ch: 'X',                  // xxx
            money: 50,
            energy: 80,
            name: "mike".to_string(), // make add
            satchel: Bag::new(),
            color: Color::RED,
            act: false,
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
    pub fn can_move(&self, next: &super::game_map::Tile /*&Vec<String>, fare: i32  */) -> bool {
        println!("item: {:?}\npos: {} - {}", self.satchel.compare_to_tile_reqs(&next.reqs), self.pos.x, self.pos.y);//xxx debug to terminal
        if self.satchel.compare_to_tile_reqs(&next.reqs) == "" && self.energy >= next.fare {
            return true
        }
        false
    }
}
// #[test]
// fn test_can_move() {
    
// }

#[test]
        fn test_player_satchel_add_count(){
            let mut player = Player::new();
            let tool = "Blue Towel".to_string();
            let tool1 = "Red Towel".to_string();
            player.add_tool(&tool);
            let has_tool = player.has_tool(&tool);
            let tool_count = player.satchel.count(&tool);
           assert_eq!(has_tool, true);
           assert_eq!(tool_count, 1);
           player.add_tool(&tool);
           player.add_tool(&tool);
           player.add_tool(&tool1);
           player.add_tool(&tool1);
           let tool_count = player.satchel.count(&tool);
           assert_eq!(tool_count, 3);
           let tool_count = player.satchel.count(&tool1);
           assert_eq!(tool_count, 2);

         }