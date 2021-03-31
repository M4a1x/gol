use crate::game::CellStatus;
use crate::game::{Cell, Game, Point};

//////////////////////////////////// Board ////////////////////////////////////

#[derive(Clone, Debug)]
struct Board {
    data: Vec<Cell>,
    cols: usize,
    rows: usize,
}

impl Board {
    fn new(cols: usize, rows: usize) -> Self {
        let mut data = Vec::with_capacity(cols * rows);
        for row in 0..rows {
            for col in 0..cols {
                let cell = Cell::new(Point::new(col, row), CellStatus::Dead);
                data.push(cell);
            }
        }

        Board { data, cols, rows }
    }

    fn alive_neighbours(&self, row: usize, col: usize) -> u8 {
        self[self.rwt(row)][self.cwl(col)].status as u8
            + self[self.rwt(row)][col].status as u8
            + self[self.rwt(row)][self.cwr(col)].status as u8
            + self[row][self.cwl(col)].status as u8
            + self[row][self.cwr(col)].status as u8
            + self[self.rwb(row)][self.cwl(col)].status as u8
            + self[self.rwb(row)][col].status as u8
            + self[self.rwb(row)][self.cwr(col)].status as u8
    }

    fn cwl(&self, col: usize) -> usize {
        if col == 0 {
            self.cols - 1
        } else {
            col - 1
        }
    }

    fn cwr(&self, col: usize) -> usize {
        if col == self.cols - 1 {
            0
        } else {
            col + 1
        }
    }

    fn rwt(&self, row: usize) -> usize {
        if row == 0 {
            self.rows - 1
        } else {
            row - 1
        }
    }

    fn rwb(&self, row: usize) -> usize {
        if row == self.rows - 1 {
            0
        } else {
            row + 1
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self[row][col].status)?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl std::ops::Index<usize> for Board {
    type Output = [Cell];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols..(index + 1) * self.cols]
    }
}

impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols..(index + 1) * self.cols]
    }
}

//////////////////////////////////// GAME ///////////////////////////////////

pub struct NaiveGame {
    main_board: Board,
    shadow_board: Board,
    shadow_board_active: bool,
    ruleset: crate::Rules,
}

impl NaiveGame {
    pub fn new(cols: usize, rows: usize, ruleset: crate::Rules) -> Self {
        Self {
            main_board: Board::new(cols, rows),
            shadow_board: Board::new(cols, rows),
            shadow_board_active: false,
            ruleset,
        }
    }

    pub fn get_width(&self) -> usize {
        self.main_board.cols
    }

    pub fn get_height(&self) -> usize {
        self.main_board.rows
    }

    fn get_active_board(&self) -> &Board {
        if self.shadow_board_active {
            &self.shadow_board
        } else {
            &self.main_board
        }
    }

    fn get_active_board_mut(&mut self) -> &mut Board {
        if self.shadow_board_active {
            &mut self.shadow_board
        } else {
            &mut self.main_board
        }
    }

    pub fn compute_next_gen(&mut self) {
        let (src, dest) = if self.shadow_board_active {
            (&self.shadow_board, &mut self.main_board)
        } else {
            (&self.main_board, &mut self.shadow_board)
        };

        for col in 0..dest.cols {
            for row in 0..dest.rows {
                dest[row][col].status =
                    Self::compute_next_cell_status(src, row, col, &self.ruleset);
            }
        }

        self.shadow_board_active = !self.shadow_board_active;
    }

    fn compute_next_cell_status(
        board: &Board,
        row: usize,
        col: usize,
        ruleset: &crate::Rules,
    ) -> CellStatus {
        let cell = board[row][col];
        let neighbour_count = board.alive_neighbours(row, col);
        match cell.status {
            CellStatus::Alive => {
                if ruleset.get_surviverule().contains(&neighbour_count) {
                    CellStatus::Alive
                } else {
                    CellStatus::Dead
                }
            }
            CellStatus::Dead => {
                if ruleset.get_birthrule().contains(&neighbour_count) {
                    CellStatus::Alive
                } else {
                    CellStatus::Dead
                }
            }
        }
    }
}

impl std::fmt::Display for NaiveGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_active_board())
    }
}

impl std::ops::Index<usize> for NaiveGame {
    type Output = [Cell];
    fn index(&self, index: usize) -> &Self::Output {
        &self.get_active_board()[index]
    }
}

impl std::ops::IndexMut<usize> for NaiveGame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.get_active_board_mut()[index]
    }
}

/////////////////////////////////// Iterator //////////////////////////////////

pub struct NaiveGameIter<'a> {
    game: &'a NaiveGame,
    curr_pos: Point,
}

impl<'a> NaiveGameIter<'a> {
    pub fn new(game: &'a NaiveGame) -> Self {
        Self {
            game,
            curr_pos: Point::default(),
        }
    }

    fn get_curr_cell_status(&mut self) -> Option<CellStatus> {
        if self.curr_pos.y >= self.game.get_active_board().rows {
            None
        } else {
            Some(self.game.get_active_board()[self.curr_pos.y][self.curr_pos.x].status)
        }
    }

    fn advance_curr_position(&mut self) {
        self.curr_pos.x += 1;

        if self.curr_pos.x >= self.game.get_active_board().cols {
            self.curr_pos.x = 0;
            self.curr_pos.y += 1;
        }
    }
}

impl<'a> Iterator for NaiveGameIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(status) = self.get_curr_cell_status() {
            match status {
                CellStatus::Dead => self.advance_curr_position(),
                CellStatus::Alive => {
                    let cell = &self.game.get_active_board()[self.curr_pos.y][self.curr_pos.x];
                    self.advance_curr_position();
                    return Some(cell);
                }
            }
        }
        None
    }
}

impl<'a> Game<'a> for NaiveGame {
    type Iter = NaiveGameIter<'a>;

    fn alive_cells(&'a self) -> Self::Iter {
        NaiveGameIter::new(&self)
    }
}

impl<'a> IntoIterator for &'a NaiveGame {
    type Item = &'a Cell;
    type IntoIter = NaiveGameIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.alive_cells()
    }
}
