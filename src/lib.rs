use parse::Rules;
use std::env;
use std::error::Error;
use std::path::Path;

mod game;
mod parse;
mod render;

use crate::game::Size;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = &config.filename {
        let pattern = parse::load_file(Path::new(filename))?;
        render::run(pattern, config.game_size)?;
    } else {
        println!("No start pattern provided!")
    }
    Ok(())
}

pub struct Config {
    filename: Option<String>,
    game_size: Size,
}

impl Config {
    pub fn new(mut args: env::Args) -> Config {
        args.next(); // Executable name

        let filename = args.next();
        let game_size = match args.next() {
            Some(arg) => panic!("Parsing game size not implemented!"),
            None => Size {
                width: 100,
                height: 100,
            },
        };

        Config {
            filename,
            game_size,
        }
    }
}
