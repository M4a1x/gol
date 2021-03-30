#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
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
