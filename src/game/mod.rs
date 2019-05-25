//game.rs
//game manager

use quicksilver::prelude::*; //reduce?
pub mod game_map;
pub mod player; 
pub mod item_bag;


pub struct Game {
    title: Asset<Image>,
    pub map: game_map::Map,
    pic: Asset<Image>,
    tileset: Asset<std::collections::HashMap<char, Image>>,
    pub player: player::Player,//vec players
    tile_size_px: Vector,
    //inventory: Asset<Image>,
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
        let map = game_map::Map::gen(20.0,20.0); // xxx get window size from main?
        //characters
        //break up into fei
        // let mut players = Vec::<player::Player>::new();
        // players.push(player::Player::new())
        let mut player = player::Player::new();
        player.add_tool(&"Blue Towel".to_string()); // ???

        let chs = "xOXo";
        let tile_size_px = Vector::new(10, 24);
        let tileset = Asset::new(Font::load(font_mono).and_then(move |text| {
            let tiles = text
                .render(chs, &FontStyle::new(tile_size_px.y, Color::WHITE))
                .expect("Could not render the font tileset.");
            let mut tileset = std::collections::HashMap::new();
            for (index, glyph) in chs.chars().enumerate() {
                let pos = (index as i32 * tile_size_px.x as i32, 0);
                let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
                tileset.insert(glyph, tile);
            }
            Ok(tileset)
        }));

        //inventory ??? or elsewhere

    Ok(Self {
        title,
        map,
        pic,
        player,
        tileset,
        tile_size_px,
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
                    .with_center((window.screen_size().x as i32 - 60, window.screen_size().y as i32 / 2 )),
                Img(&image),
            );
            Ok(())
        })?;


        // Draw the map
        let tile_size_px = Vector::new(24, 24);
        let offset_px = Vector::new(50, 120);
        let (tileset, map, p_pos, p_ch) = (&mut self.tileset, &self.map, &self.player.pos, &self.player.ch);
        tileset.execute(|tileset| {
            for tile in map.map.iter() {
                if tile.pos != *p_pos {
                if let Some(image) = tileset.get(&tile.ch) {
                    let pos_px = tile.pos.times(tile_size_px);
                    window.draw(
                        &Rectangle::new(offset_px + pos_px, image.area().size()),
                        Blended(&image, tile.color),
                    );
                }
               else { // xxx how to ???
                   if let Some(image) = tileset.get(&'X') {
                    let pos_px = p_pos.times(tile_size_px);
                    window.draw(
                        &Rectangle::new(offset_px + pos_px, image.area().size()),
                        Blended(&image, Color::RED),
                    );
                }
               } 
            }
            }
            Ok(())
        })?;

        let p1 = &self.player;
        let max_bar = 100.0;
        let curr_power = p1.energy as f32;
        let curr_money = p1.money as f32; // xxx make min/max
        let map_size_px = self.map.size.times(tile_size_px);
        let power_bar_pos_px = offset_px + Vector::new(map_size_px.x, 0.0);
        let money_bar_pos_px = offset_px + Vector::new(map_size_px.x, tile_size_px.y + 5.0);
        //draw max vals shaded
        window.draw(
            &Rectangle::new(power_bar_pos_px, (max_bar, tile_size_px.y)),
            Col(Color::BLUE.with_alpha(0.5)),
            );
        window.draw(
            &Rectangle::new(money_bar_pos_px, (max_bar, tile_size_px.y)),
            Col(Color::GREEN.with_alpha(0.5)),
        );
        //draw curr values on top
        window.draw(
            &Rectangle::new(power_bar_pos_px, (curr_power, tile_size_px.y)),
            Col(Color::BLUE),
        );
        window.draw(
            &Rectangle::new(money_bar_pos_px, (curr_money, tile_size_px.y)),
            Col(Color::GREEN),
        );

//
    Ok(())
    }
}
//end impl state for game