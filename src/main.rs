// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]

#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]



#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate ncurses;
extern crate rand;

// #[macro_use]
extern crate log;
extern crate env_logger;

mod geometry;

mod ball;
mod paddle;
mod board;
mod score;
mod game;
mod ui;
mod io;
mod state;

use game::Game;

fn main() {
    env_logger::init().unwrap();

    let mut game = Game::new();
    let mut i = 0;
    loop {
        i += 1;
        game.tick(i);
    }
    // println!("Thanks for playing!");
}
