mod board;
mod game;

use game::Game;
use board::Cell;

use std::time::Instant;
use cpuprofiler::PROFILER;

pub fn main () {
    let mut game = Game::new(1000, 1000);
    game[0][1] = Cell::Alive;
    game[1][2] = Cell::Alive;
    game[2][0] = Cell::Alive;
    game[2][1] = Cell::Alive;
    game[2][2] = Cell::Alive;

    PROFILER.lock().unwrap().start("../profiling/naive.profile").unwrap();
    let now = Instant::now();

    for _i in 1..1000 {
        game.next_gen();
    }

    println!("{:?}", now.elapsed());
    PROFILER.lock().unwrap().stop().unwrap();
}