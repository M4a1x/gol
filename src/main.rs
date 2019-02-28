mod board;
mod game;

use std::env;
use game::Game;
use board::Cell;

fn main () {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let width = 10;
    let height = 10;

    let mut game = Game::new(width, height);
    game[0][1] = Cell::Alive;
    game[1][2] = Cell::Alive;
    game[2][0] = Cell::Alive;
    game[2][1] = Cell::Alive;
    game[2][2] = Cell::Alive;

    println!("{}", game);

    loop {
        println!("{}", game.next_gen());
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}