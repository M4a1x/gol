use std::env;
use std::error::Error;

use gol::{self, Config};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new(env::args())?;
    gol::run(config)
}
