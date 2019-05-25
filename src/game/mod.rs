//game.rs
//game manager
//move to main?
use quicksilver::prelude::*; //reduce?
mod game_map; // make mod.rs ???
mod player; // make mod.rs ???
mod item_bag;
// mod map;
// mod tile;

pub struct Game {
    title: Asset<Image>,
    map: game_map::Map,
    pic: Asset<Image>,
    //tileset: Asset<std::collections::HashMap<char, Image>>,
    pub player: player::Player,//vec players
    //...
}

impl State for Game { //qs state trait handles window rendering  
    fn new() -> Result<Self> {
        //switch fmt
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
        //players
        //break up into fei
        // let mut players = Vec::<player::Player>::new();
        // players.push(player::Player::new())
        let mut player = player::Player::new();
        player.add_tool(&"Blue Towel".to_string()); // ???


    Ok(Self {
        title,
        map,
        pic,
        player,
        })
    }
        /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;

        if window.keyboard()[Key::Left] == Pressed {
                    self.player.pos.x -= 1.0;
                }
                if window.keyboard()[Key::Right] == Pressed {
                    self.player.pos.x += 1.0;
                }
                if window.keyboard()[Key::Up] == Pressed {
                    self.player.pos.y -= 1.0;
                }
                if window.keyboard()[Key::Down] == Pressed {
                    self.player.pos.y += 1.0;
                }
                if window.keyboard()[Key::A] == Pressed {
                    self.player.money -= 1; // xxx
                }
                if window.keyboard()[Key::S] == Pressed {
                    self.player.money += 1; // xxx
                }
                if window.keyboard()[Key::Z] == Pressed {
                    self.player.energy -= 1; // xxx
                }
                if window.keyboard()[Key::X] == Pressed {
                    self.player.energy +=  1; // xxx
                }
                if window.keyboard()[Key::Escape].is_down() {
                    window.close();
                }
        Ok(())//ret ok void
    }

        //...
        //draw everything

        fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        // Draw the game title
        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;

        self.pic.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 4, window.screen_size().y as i32 / 4 )),
                Img(&image),
            );
            Ok(())
        })?;


        // Draw the map
        

        // Draw player
        

    Ok(())
    }
}