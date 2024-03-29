//game.rs
//game manager

use quicksilver::prelude::*;
pub mod game_map;
pub mod item_bag;
pub mod player;
pub mod store;

pub struct Game {
    title: Asset<Image>,
    pub map: game_map::Map,
    pic: Asset<Image>,
    tileset: Asset<std::collections::HashMap<char, Image>>,
    pub player: player::Player, //vec players
    //tile_size_px: Vector,
    controls: Asset<Image>,
    msg: String,
    display_msg: bool,
    msg_asset: Asset<Image>,
    store: store::Store,
    store_asset: Asset<Image>,
    bees: Asset<Image>,
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
                "Conrols:\nUp: North\nDown: South\nLeft: East\nRight: West\nW: act  M: market\nEsc: quit",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        //store display
        let store = store::Store::gen_store();
        let store_clone = store.clone();
        let store_asset = Asset::new(Font::load(font_mono).and_then(move |font| {
            font.render(
                &store_clone.contents_to_strings().join("\n"),
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));
        //message
        let msg = "Missing: ".to_string();
        let display_msg = false;
        let msg_clone = msg.clone();
        let msg_asset =
            Asset::new(Font::load(font_mono).and_then(move |font| {
                font.render(&msg_clone, &FontStyle::new(20.0, Color::BLACK))
            }));
        //pic for experimenting
        let pic = Asset::new(Image::load("testimg1.png"));
        let bees = Asset::new(Image::load("bees.jpeg"));
        //map
        let map = game_map::Map::gen(25, 25); // xxx use window size?
                                              //characters
                                              //break up into fei
                                              // let mut players = Vec::<player::Player>::new();
                                              // players.push(player::Player::new())
        let mut player = player::Player::new();
        player.add_tool(&"Blue towel".to_string());
        // ttt ???
        //based on tse example
        let chs = "amoxwlg";
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
    
      

        Ok(Self {
            title,
            map,
            pic,
            player,
            tileset,
            controls,
            msg,
            display_msg,
            msg_asset,
            store,
            store_asset,
            bees,
        })
    }

    // Process keyboard update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;
        let mut curr_pos = self.player.pos;
        let mut moved = false;

        if window.keyboard()[Key::Left] == Pressed {
            //is down? {
            curr_pos.x -= 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(curr_pos))
            {
                //compare tile requirements to player's items
                moved = true;
            }
        }
        if window.keyboard()[Key::Right] == Pressed {
            curr_pos.x += 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(curr_pos))
            {
                //rewire to player bag to tile reqs ???
                moved = true;
            }
        }
        if window.keyboard()[Key::Up] == Pressed {
            curr_pos.y -= 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(curr_pos))
            {
                moved = true;
            }
        }
        if window.keyboard()[Key::Down] == Pressed {
            curr_pos.y += 1.0;
            if self.map.is_on_board(curr_pos) && self.player.can_move(&self.map.get_tile(curr_pos))
            {
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
            } else {
                self.player.act = true;
            }
        }
        //activate store window
        if window.keyboard()[Key::M] == Pressed {
            if self.store.is_active {
                self.store.is_active = false;
            } else {
                self.store.is_active = true;
            }
        }
        if window.keyboard()[Key::R] == Pressed {
            // xxx add rope
            if self.store.purchase("(R)ope", &mut self.player.money)  {
                self.player.add_tool(&"Rope".to_string());
            }
        }
        if window.keyboard()[Key::F] == Pressed {
            if self.store.purchase("(F)ace", &mut self.player.money)  {
                self.player.add_tool(&"Face".to_string());
            }
        }
        if window.keyboard()[Key::B] == Pressed {
            if self.store.purchase("(B)oat", &mut self.player.money)  {
                self.player.add_tool(&"Boat".to_string());
            }
        }
        if window.keyboard()[Key::Escape].is_down() {
            window.close();
        }
        //update player if move successful
        if moved {
            if self.map.get_tile(curr_pos).ch == 'g' {
                self.map.win = true; // xxx ???
                window.close(); // xxx end game
                //game won window, prompt to press esc to exit
            }
            self.player.pos = curr_pos;
            self.player.energy -= self.map.get_tile(curr_pos).fare;
            //self.map.get_mut_tile(curr_pos).seen = true;
            self.player.money += self.map.get_tile(curr_pos).chance_val;
            self.display_msg = false;
            self.msg.clear();

            //update tiles
            self.map.unshroud_dis_x(curr_pos, 3); //sets tiles within range x to seen (they are displayed)
                                                  //print current state to terminal // xxx disable
            self.dump_stats();
        } else {
            //return missing item for display
            if self.player.pos != curr_pos {
                //if a move was attempted
                self.msg = self
                    .player
                    .satchel
                    .compare_to_tile_reqs(&self.map.get_tile(curr_pos).reqs); //gets missing item from tile
                self.display_msg = true;
                self.dump_stats();
            }
        }

        Ok(()) //ret ok void
    }

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
        if self.player.act {
            // xxx
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

        // if won display game won ... and end or last on top of everything, prompt for esc

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
                if let Some(image) = tileset.get(&tile.get_display_ch()) {
                    let pos_px = tile.pos.times(tile_size_px);
                    window.draw(
                        &Rectangle::new(offset_px + pos_px, image.area().size()),
                        Blended(&image, tile.get_display_color()),
                    );
                }
            }
            Ok(())
        })?;

        //Draw Player
        let (tileset, p1) = (&mut self.tileset, &self.player);
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
                &image.area().with_center((
                    window.screen_size().x as i32 - 75,
                    window.screen_size().y as i32 - 100,
                )),
                Img(&image),
            );
            Ok(())
        })?;

        //draw store ???
        if self.store.is_active {
            self.store_asset.execute(|image| {
                window.draw(
                    &image.area().with_center((
                        window.screen_size().x as i32 - 75,
                        window.screen_size().y as i32 - 250,
                    )),
                    Img(&image),
                );
                Ok(())
            })?;
        }

        let p1 = &self.player;
        let max_bar = 100.0;
        let curr_power = p1.energy; //add checks
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

        //msg alert for wasm page xxx
        let act_width_px = if self.display_msg {
            tile_size_px.x
        } else {
            0.0
        };

        window.draw(
            &Rectangle::new(
                money_bar_pos_px + tile_size_px,
                (act_width_px, tile_size_px.y),
            ),
            Col(Color::BLUE),
        );
        //draw inventory
        let font_mono = "FreeMono.ttf"; // xxx new font
        let mut player_bag = "Inventory:\n".to_string();
        player_bag.push_str(&self.player.contents_to_string());
        let mut inventory =
            Asset::new(Font::load(font_mono).and_then(move |font| {
                font.render(&player_bag, &FontStyle::new(20.0, Color::BLACK))
            }));
        inventory.execute(|image| {
            window.draw(
                // &image.area()
                // .translate(inventory_pos_px),
                &Rectangle::new(inventory_pos_px, image.area().size()),
                Img(&image),
            );
            Ok(())
        })?;
        //draw msg, if exists
        let mut missing = "Missing: ".to_string();
        missing.push_str(&self.msg);

        self.msg_asset = Asset::new(
            Font::load(font_mono)
                .and_then(move |font| font.render(&missing, &FontStyle::new(20.0, Color::BLACK))),
        );
        if self.display_msg {
            self.msg_asset.execute(|image| {
                window.draw(
                    &image
                        .area()
                        .with_center((window.screen_size().x as i32 / 2, 80)),
                    Img(&image),
                );
                Ok(())
            })?;
        } //msg

        // draw bees on top
        if self.display_msg && self.player.act {
            self.bees.execute(|image| {
                window.draw(
                    &image
                        .area()
                        .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                    Img(&image),
                );
                Ok(())
            })?;
        } 

        //
        Ok(())
    }
}//impl state for game

impl Game {
    //dump stats to terminal on move xxx
    pub fn dump_stats(&self) {
        println!(
            "win: {}\nPpos: {} - {}\nTpos: {} - {} id: {}  seen: {}\npow: {}\nmoney: {}\n",
            self.map.win,
            self.player.pos.x,
            self.player.pos.y,
            self.map.get_tile(self.player.pos).pos.x,
            self.map.get_tile(self.player.pos).pos.y,
            self.map.get_tile(self.player.pos).id,
            self.map.get_tile(self.player.pos).seen,
            self.player.energy,
            self.player.money
        );
    }
}

