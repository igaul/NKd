//game.rs
//game manager
//move to main?
use quicksilver::prelude::*; //reduce?
#[path = "game_map.rs"] mod game_map; // make mod.rs ???

pub struct Game {
    title: Asset<Image>,
    map: game_map::Map,
    pic: Asset<Image>,
    //player inventory
    //...
}

impl State for Game { //qs state trait handles window rendering  
    fn new() -> Result<Self> {
        // gnu mono
        let font_mono = "FreeMono.ttf"; // xxx new font / ss
        //title
        let title = Asset::new(Font::load(font_mono).and_then(|font| {
                font.render("NKd", &FontStyle::new(72.0, Color::BLACK))
            }));
        //pic for experimenting 
        let pic = Asset::new(Image::load("testimg1.png"));
        //map
        let map = game_map::Map::new(20,20);
Ok(Self {
            title,
            map,
            pic,
        })
    }
        /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;


                  
    Ok(())
    }

        //...
        //draw everything
}