# NKd
#####
A simple game to explore rust, wasm, and quicksilver.

### Topic Area:
Game
#
### Vision:
Two-dimensional game with keyboard control using rust and web assembly. Player can move, interact with items, collect points, complete and possibly save levels. Beware of bees.
#
### Credits
Quicksilver [crate](https://crates.io/crates/quicksilver), and it's many dependancies, wasm and rust communities. 

Tomas Sedovic's [quicksilver tutorial](https://github.com/tomassedovic/quicksilver-roguelike)
a friendly introduction to using quicksilver and rendering games. 


#
### Use:


#### [click for demo](https://igaul.github.io/NKd)
    Note: some extensions break js rendering canvas, private modes of firefox on windows and chrome on linux appear to consistently work. 


#### Linux:
To build for Ubuntu 18LTS targeting wasm you need:

* rust

    `curl https://sh.rustup.rs -sSf | sh`
*  gcc (to compile proc-macro2), libssl (for cargoweb) 

    `sudo apt install gcc libssl-dev `
* webassembly (supported bytecode)

    `rustup target add wasm32-unknown-unknown`
* cargo-web (does all the wasm-js stuff for us!)

    `cargo install cargo-web`
* clone this repo (or download files)

    `git clone https://github.com/igaul/NKd.git`


##### Web browser


* compile

    `cargo web start` to test locally (`deploy` to just build files)


##### Linux computer

* compile

    `cargo run --release`
    

(For other distros there are some dependancy conflicts)
Some features are only availible in cargo (rustc) compilation, not in browser version, such as inventory updates, due to rerendering image.

##### Game Play
Move around the board encountering obstacles and puzzles, searching for a gem on each level, collecting gem wins level.
Helpful items can be purchased from the market.

##### Current State
Sticking with one codebase for native and browser forces some design changes along with inexperience creating games, the wasm version does not currently display missing items or full inventory as in linux version. Was not able to make Atlas functional.

Long term project vision include more levels, terrain, and chalanges, along with game menu and settings menu. Making separate pages(menu, game, win/lose), etc...

