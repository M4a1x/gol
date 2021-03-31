use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod error;
pub mod life;
pub mod rules;

pub use error::ParseError;
pub use rules::Rules;

pub fn load_file(filepath: &Path) -> Result<Pattern, ParseError> {
    match filepath
        .extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.to_lowercase())
        .as_deref()
    {
        Some("life") | Some("lif") => life::parse(File::open(filepath)?),
        Some("rle") => panic!("Not yet implemented!"),
        Some("l") => panic!("Not yet implemented!"),
        Some("plf") => panic!("Not yet implemented!"),
        Some("mcl") => panic!("Not yet implemented!"),
        Some(all) => Err(ParseError::UnknownFileExtension(all.to_owned())),
        None => Err(ParseError::UnknownFileExtension(
            "file has no extension".to_owned(),
        )),
    }
}

pub trait Parser {
    fn parse(input: impl Read) -> Result<Pattern, ParseError>;
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub size: Size,
    pub alive_list: Vec<Cell>,
    pub config: PatternConfig,
}

#[derive(Debug, PartialEq)]
pub struct PatternConfig {
    pub ruleset: Option<Rules>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub wrap_edges: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub status: CellStatus,
    pub pos: Point,
}

impl Cell {
    pub fn new(pos: Point, status: CellStatus) -> Self {
        Cell { pos, status }
    }

    pub fn new_alive(x: usize, y: usize) -> Self {
        Cell {
            pos: Point::new(x, y),
            status: CellStatus::Alive,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellStatus {
    Dead = 0,
    Alive = 1,
}
