// TODO: Make interface for Displaying different games (naive, sparse, hashtable, ...)
// TODO: Move traits to respective modules?
// common stuff is currently in util/traits.rs
use crate::parser::{ParseError, Pattern};
use crate::util::Cell;

// pub trait Game {
//     alive_cells(&self) -> List<Cell>;
//     next(&mut self);
// }

// pub trait Parser {
//     fn parse(input: impl Read) -> Result<Pattern, ParseError>;
// }
