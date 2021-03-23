use parser::Rules;
use std::env;
use std::error::Error;
use std::path::Path;

mod naive;
mod parser;

pub struct Config {
    filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let pattern = parser::load_file(Path::new(&config.filename));
    run_naive();
    Ok(())
}

fn run_naive() {
    use naive::*;
    use std::thread;
    use std::time;
    let mut game = Game::new(5, 5, Rules::default());

    game[0][1] = Cell::Alive;
    game[1][2] = Cell::Alive;
    game[2][0] = Cell::Alive;
    game[2][1] = Cell::Alive;
    game[2][2] = Cell::Alive;

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
