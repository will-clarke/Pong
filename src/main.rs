#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate ncurses;


mod geometry;

mod ball;
mod paddle;
mod board;
mod score;
mod config;
mod game;
mod ui;
mod io;

use game::Game;

fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    loop {
        game.tick();
    }
}
