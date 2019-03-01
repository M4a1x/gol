mod board;
mod game;

use game::Game;
use board::Cell;

use cpuprofiler::PROFILER;

fn main () {
    let mut game = Game::new(1000, 1000);
    game[0][1] = Cell::Alive;
    game[1][2] = Cell::Alive;
    game[2][0] = Cell::Alive;
    game[2][1] = Cell::Alive;
    game[2][2] = Cell::Alive;

    PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();

    for _i in 1..1000 {
        game.next_gen();
    }

    PROFILER.lock().unwrap().stop().unwrap();
}