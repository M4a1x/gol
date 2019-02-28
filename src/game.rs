use super::board::Board;
use super::board::Cell;

enum ActiveBoard {
    BoardOne,
    BoardTwo
}

pub struct Game {
    b1: Board,
    b2: Board,
    active: ActiveBoard
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        Game {
            b1: Board::new(cols, rows),
            b2: Board::new(cols, rows),
            active: ActiveBoard::BoardOne
        }
    }

    pub fn get_active(&self) -> &Board {
        match self.active {
            ActiveBoard::BoardOne => &self.b1,
            ActiveBoard::BoardTwo => &self.b2
        }
    }

    fn get_active_mut(&mut self) -> &mut Board {
        match self.active {
            ActiveBoard::BoardOne => &mut self.b1,
            ActiveBoard::BoardTwo => &mut self.b2
        }
    }

    pub fn next_gen(&mut self) -> &Board {
        match self.active {
            ActiveBoard::BoardOne => { 
                self.b1.write_next_gen(&mut self.b2);
                self.active = ActiveBoard::BoardTwo;
                &self.b2
            },
            ActiveBoard::BoardTwo => { 
                self.b2.write_next_gen(&mut self.b1);
                self.active = ActiveBoard::BoardOne;
                &self.b1
            }
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_active())
    }
}

impl std::ops::Index<usize> for Game {
    type Output = [Cell];
    fn index(&self, index: usize) -> &Self::Output {
        &self.get_active()[index]
    }
}

impl std::ops::IndexMut<usize> for Game {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        &mut self.get_active_mut()[index]
    }
}