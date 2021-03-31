pub mod naive;

pub trait Game<'a> {
    // TODO: Instead, mayble implement Iterator struct and .iter() method on game and &game
    // Maybe remove IntoIterator supertrait, since the game should not be turned into an iterator,
    // but rather provide an immutable iterator over the alive cells, i.e. a convenience call for
    // alive_cells, such that `for alive_cell in my_game` works.
    //type Item: AsRef<Cell>;
    type Iter: Iterator<Item = &'a Cell>; // called an associated type

    fn alive_cells(&'a self) -> Self::Iter;
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

impl std::fmt::Display for CellStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::Alive => "*",
            Self::Dead => ".",
        };
        write!(f, "{}", s)
    }
}
