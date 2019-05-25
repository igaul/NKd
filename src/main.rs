//main
//
//
//
//
////
/// 

extern crate quicksilver;

pub mod game_map;
pub mod game;
pub mod item_bag;
/////mod tile;
mod player;




fn main() {

    // NOTE: Set HIDPI to 1.0 to get pixel-perfect rendering.
    // Otherwise the window resizes to whatever value the OS sets and
    // scales the contents.
    // https://docs.rs/glutin/0.19.0/glutin/dpi/index.html
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let settings = quicksilver::lifecycle::Settings {
        // If the graphics do need to be scaled (e.g. using
        // `with_center`), blur them. This looks better with fonts.
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };

    quicksilver::lifecycle::run::<game::Game>("NKd", quicksilver::geom::Vector::new(800, 600), settings);

}
