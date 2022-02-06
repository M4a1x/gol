use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

pub mod error;
pub mod life;
pub mod rules;

pub use error::ParseError;
pub use rules::Rules;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(' ').collect();
        if coords.len() != 2 {
            return Err(ParseError::InvalidFormat(s.into()));
        }

        let x_fromstr = coords[0].parse::<isize>()?;
        let y_fromstr = coords[1].parse::<isize>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellStatus {
    Dead = 0,
    Alive = 1,
}

impl Default for CellStatus {
    fn default() -> Self {
        CellStatus::Dead
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

    pub fn from_coords(x: isize, y: isize) -> Self {
        Cell {
            pos: Point::new(x, y),
            status: CellStatus::Alive,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub alive: Vec<Cell>,
    pub config: Config,
}

impl Pattern {
    pub fn new(alive: Vec<Cell>, config: Config) -> Self {
        Pattern { alive, config }
    }
    pub fn size(&self) -> Size {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for cell in &self.alive {
            if cell.status == CellStatus::Alive {
                if cell.pos.x > max_x {
                    max_x = cell.pos.x;
                }
                if cell.pos.x < min_x {
                    min_x = cell.pos.x;
                }
                if cell.pos.y > max_y {
                    max_y = cell.pos.y;
                }
                if cell.pos.y < min_y {
                    min_y = cell.pos.y;
                }
            }
        }
        Size {
            // TODO: Make this safe for the whole range
            width: (max_x - min_x) as usize + 1,
            height: (max_y - min_y) as usize + 1,
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Config {
    pub ruleset: Option<Rules>,
    pub description: Vec<String>,
    pub author: Option<String>,
    pub wrap_edges: bool,
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
