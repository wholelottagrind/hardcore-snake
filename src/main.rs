mod direction;
mod point;
mod snake;
mod board;
mod game;
mod terminal_handler;
mod config;
mod maze;

use game::Game;
use std::io;

fn main() -> io::Result<()> {
    let mut game = Game::new(32, 16);
    game.start()?;
    
    Ok(())
}
