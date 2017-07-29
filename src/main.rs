#[allow(dead_code)]
#[allow(unused_variables)]

extern crate num;

mod geometry;

mod ball;
mod paddle;
mod board;
mod score;
mod config;
mod game;

use game::Game;

fn main() {
    println!("Hello, world!");
    let game = Game::new();
}
