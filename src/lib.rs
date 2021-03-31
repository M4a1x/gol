use parse::Rules;
use std::env;
use std::error::Error;
use std::path::Path;

mod game;
mod parse;
mod render;

pub struct Config {
    filename: Option<String>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = &config.filename {
        let pattern = parse::load_file(Path::new(filename))?;
    }
    render::run()?;
    //run_naive();
    Ok(())
}

impl Config {
    pub fn new(mut args: env::Args) -> Config {
        args.next(); // Executable name

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Config { filename: None },
        };

        Config {
            filename: Some(filename),
        }
    }
}
