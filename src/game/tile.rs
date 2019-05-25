//tile.rs
//tile structure, etc
//may push into map module

use quicksilver::prelude::*;
//use item_bag::item_bag;

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