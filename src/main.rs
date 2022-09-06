mod snake;
mod direction;
mod game;
mod point;
mod command;

use std::io::stdout;
use crate::game::Game;

fn main() {
    println!("Hello, world!");

    Game::new(stdout(), 10, 10).run();
}
