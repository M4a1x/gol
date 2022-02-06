use parser::{Pattern, Rules};
use std::env;
use std::error::Error;
use std::path::Path;

mod naive;
mod parser;

pub struct Config {
    filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let pattern = parser::load_file(Path::new(&config.filename))?;
    run_naive(pattern);
    Ok(())
}

fn run_naive(pattern: Pattern) {
    use naive::*;
    use std::thread;
    use std::time;
    let gamesize = pattern.size();
    let mut game = Game::new(
        gamesize.width,
        gamesize.height,
        pattern.config.ruleset.unwrap_or_default(),
    );

    let min_x = pattern
        .alive
        .iter()
        .min_by_key(|cell| cell.pos.x)
        .unwrap()
        .pos
        .x;
    let min_y = pattern
        .alive
        .iter()
        .min_by_key(|cell| cell.pos.y)
        .unwrap()
        .pos
        .y;

    // Move pattern to the top left corner of the game world
    for cell in pattern.alive {
        let x = cell.pos.x - min_x;
        let y = cell.pos.y - min_y;
        game[y as usize][x as usize] = Cell::Alive;
    }

    loop {
        println!("{}", game);
        game.compute_next_gen();
        thread::sleep(time::Duration::from_millis(100));
    }
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Executable name

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { filename })
    }
}
