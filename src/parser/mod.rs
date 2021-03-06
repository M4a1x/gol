use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

pub mod error;
pub mod life;
pub mod rules;

pub use error::ParseError;
pub use rules::Rules;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    status: CellStatus,
    pos: Point,
}

impl Cell {
    pub fn new(pos: Point, status: CellStatus) -> Self {
        Cell { pos, status }
    }

    pub fn new_alive(x: u32, y: u32) -> Self {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    size: Size,
    alive_list: Vec<Cell>,
    config: PatternConfig,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq)]
pub struct PatternConfig {
    ruleset: Option<Rules>,
    description: Option<String>,
    author: Option<String>,
    wrap_edges: bool,
}

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
