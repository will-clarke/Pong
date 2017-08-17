// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate ncurses;
extern crate rand;


mod geometry;

mod ball;
mod paddle;
mod board;
mod score;
mod config;
mod game;
mod ui;
mod io;
mod state;

use game::Game;

fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    let mut i = 0;
    loop {
        i += 1;
        game.tick(i);
    }
}
