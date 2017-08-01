#[allow(dead_code)]
#[allow(unused_variables)]

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
    for i in 1..100 {
        game.tick();
    }
}
