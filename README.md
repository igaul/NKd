# NKd
#### sp19


### Topic Area:
Game
#
### Vision:
Two-dimensional game with keyboard control using rust and web assembly. Player can move, interact with items, collect points, complete and maybe save levels. Beware of bees.
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

#####Web

* compile

    `cargo web start` to test locally (deploy to just build files)
#####Linux
* compile

    `cargo run`
    

(For other distros there are some dependancy conflicts)
