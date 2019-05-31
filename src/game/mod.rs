//game.rs
//game manager

use quicksilver::prelude::*;
pub mod game_map;
pub mod item_bag;
pub mod player;

pub struct Game {
    title: Asset<Image>,
    pub map: game_map::Map,
    pic: Asset<Image>,
    tileset: Asset<std::collections::HashMap<char, Image>>,
    tileset_upper: Asset<std::collections::HashMap<char, Image>>,
    pub player: player::Player, //vec players
    //tile_size_px: Vector,
    controls: Asset<Image>,
    msg: String,
    display_msg: bool,
    //inventory: Asset<Image>,
    //...
}


impl State for Game {
    //qs state trait handles window rendering
    fn new() -> Result<Self> {
        //switch fmt
        // gnu mono
        let font_mono = "FreeMono.ttf"; // xxx new font / ss
                                        //title
        let title = Asset::new(
            Font::load(font_mono)
                .and_then(|font| font.render("NKd", &FontStyle::new(72.0, Color::BLACK))),
        );
        //controls
        let controls = Asset::new(Font::load(font_mono).and_then(move |font| {
            font.render(
                "Conrols:\nUp: North\nDown: South\nLeft: East\nRight: West\nA/S: h\nZ/X: p\nW: act  R: rope\nEsc: quit",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));
        //message
        let msg = "".to_string();
        let display_msg = false;
        //pic for experimenting
        let pic = Asset::new(Image::load("testimg1.png"));
        //map
        let map = game_map::Map::gen(20, 20); // xxx use window size?
                                                  //characters
                                                  //break up into fei
                                                  // let mut players = Vec::<player::Player>::new();
                                                  // players.push(player::Player::new())
        let mut player = player::Player::new();
        player.add_tool(&"Blue towel".to_string());
         // T ???
        

        let chs = "amoxl";
        let tile_size_px = Vector::new(10,24);
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
        let chs = "XALMO";
        let tile_size_px = Vector::new(10,24);
        let tileset_upper = Asset::new(Font::load(font_mono).and_then(move |text| {
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
            tileset_upper,
            //tile_size_px,
            controls,
            msg,
            display_msg,
        })
    }

    

    /// Process keyboard update the game state //move to player ???
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;
        let mut curr_pos = self.player.pos;
        let mut moved = false;

        if window.keyboard()[Key::Left] == Pressed {//is down? {
            curr_pos.x -= 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(&curr_pos)){ //compare tile requirements to player's items
                moved = true;
            }
            
        }
        if window.keyboard()[Key::Right] == Pressed {
            curr_pos.x += 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(&curr_pos)){ //rewire to player bag to tile reqs ???
                 moved = true;
            }
           
        }
        if window.keyboard()[Key::Up] == Pressed {
            curr_pos.y -= 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(&curr_pos)) {
                 moved = true;
            }
            
        }
        if window.keyboard()[Key::Down] == Pressed {
            curr_pos.y += 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(&curr_pos)) {
                 moved = true;
            }
        }
        if window.keyboard()[Key::A].is_down() {
            self.player.money -= 10; // xxx
            if self.player.money < 0 {
                self.player.money = 0;
            }
        }
        if window.keyboard()[Key::S].is_down() {
            self.player.money += 10; // xxx
        }
        if window.keyboard()[Key::Z] == Pressed {
            self.player.energy -= 10; // xxx
            if self.player.energy < 0 {
                self.player.energy = 0;
            }
        }
        if window.keyboard()[Key::X] == Pressed {
            self.player.energy += 10; // xxx
        }
        if window.keyboard()[Key::W] == Pressed {
            if self.player.act {
                self.player.act = false;
            }
            else {
                self.player.act = true;
            }
        }
        if window.keyboard()[Key::R] == Pressed {// xxx add rope
            if self.player.money >= 50 {
            self.player.add_tool(&"Rope".to_string());
            self.player.money -= 50;
            }
        }
        if window.keyboard()[Key::Escape].is_down() {
            window.close();
        }
        //update player if move successful
        if moved {
            self.player.pos = curr_pos;
            self.player.energy -=  self.map.get_tile(&curr_pos).fare;
            self.map.get_mut_tile(curr_pos).seen = true;
             self.display_msg = false;
             self.msg.clear();
            self.dump_stats();
        }
        else { //return missing item for display
            if self.player.pos != curr_pos { //if a move was attempted
                self.msg = self.player.satchel
                            .compare_to_tile_reqs(&self.map.get_tile(&curr_pos).reqs);//gets missing item from tile
                self.display_msg = true;
                self.dump_stats();
            }
        }
        

        Ok(()) //ret ok void
    }

    //...
    ///draw everything

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        // Draw title
        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;
        //draw image if activated
        if self.player.act { // xxx
            self.pic.execute(|image| {
                window.draw(
                    &image.area().with_center((
                        window.screen_size().x as i32 - 100,
                        window.screen_size().y as i32 / 2,
                    )),
                    Img(&image),
                );
                Ok(())
            })?;
        }

        // Draw map
        let tile_size_px = Vector::new(24, 24);
        let map_size_px = self.map.size.times(tile_size_px);
        let offset_px = Vector::new(50, 120);
        let (tileset, map, _p_pos, _p_ch) = (
            &mut self.tileset,
            &self.map,
            &self.player.pos,
            &self.player.ch,
        );
        tileset.execute(|tileset| {
            for tile in map.map.iter() {
                if let Some(image) = tileset.get(&tile.ch) {
                    let pos_px = tile.pos.times(tile_size_px);
                    window.draw(
                        &Rectangle::new(offset_px + pos_px, image.area().size()),
                        Blended(&image, tile.color),
                    );
                }
            }
            Ok(())
        })?;

        //Draw Player
        let (tileset, p1) = (&mut self.tileset_upper, &self.player);
        tileset.execute(|tileset| {
            if let Some(image) = tileset.get(&p1.ch) {
                let pos_px = offset_px + p1.pos.times(tile_size_px);
                window.draw(
                    &Rectangle::new(pos_px, image.area().size()),
                    Blended(&image, p1.color),
                );
            }
            Ok(())
        })?;

        //draw button controls
        self.controls.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 - 100, window.screen_size().y as i32 - 100)),
                Img(&image),
            );
            Ok(())
        })?;

        let p1 = &self.player;
        let max_bar = 100.0;
        let curr_power = p1.energy;//add checks
        let curr_money = p1.money; // xxx make min/max
        let power_bar_pos_px = offset_px + Vector::new(map_size_px.x, 0.0);
        let money_bar_pos_px = offset_px + Vector::new(map_size_px.x, tile_size_px.y);
        let inventory_pos_px = offset_px + Vector::new(map_size_px.x, tile_size_px.y * 2.0);
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
        //draw inventory
        let font_mono = "FreeMono.ttf"; // xxx new font
        let mut player_bag = "Inventory:\n".to_string();
        player_bag.push_str(&self.player.contents_to_string());
        let mut inventory = Asset::new(Font::load(font_mono).and_then(move |font| {
            font.render(
                &player_bag,
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));
        inventory.execute(|image| {
            window.draw(
                &image.area()
                .translate(inventory_pos_px),
                Img(&image),
            );
            Ok(())
        })?;
        //draw msg, if exists
        if self.display_msg {
            let mut missing = "Missing: ".to_string();
            missing.push_str(&self.msg);
            let mut missing_asset = Asset::new(Font::load(font_mono).and_then(move |font| {
                font.render(
                    &missing,
                    &FontStyle::new(20.0, Color::BLACK),
                )
            }));
            missing_asset.execute(|image| {
                window.draw(
                    &image.area()
                    .with_center((window.screen_size().x as i32 / 2, 80)),
                    Img(&image),
                );
                Ok(())
            })?;
        }//msg
        
        //
        Ok(())
    }
}
//end impl state for game

impl Game {
    //dump stats to terminal on move xxx
    pub fn dump_stats(&self) {
        println!("\nPpos: {} - {}\nTpos: {} - {} id: {}  seen: {}\npow: {}\nmoney: {}\n",  
        self.player.pos.x, self.player.pos.y,
        self.map.get_tile(&self.player.pos).pos.x,
        self.map.get_tile(&self.player.pos).pos.y,
        self.map.get_tile(&self.player.pos).id,
        self.map.get_tile(&self.player.pos).seen,
        self.player.energy,
        self.player.money
        );
    }

}
//tests or in test file