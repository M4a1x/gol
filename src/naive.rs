#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Cell::Alive => "*",
            Cell::Dead => ".",
        };
        write!(f, "{}", s)
    }
}

//////////////////////////////////// Board ////////////////////////////////////

#[derive(Clone, Debug)]
struct Board {
    data: Vec<Cell>,
    cols: usize,
    rows: usize,
}

impl Board {
    fn new(cols: usize, rows: usize) -> Self {
        Board {
            data: vec![Cell::Dead; cols * rows],
            cols,
            rows,
        }
    }

    fn alive_neighbours(&self, row: usize, col: usize) -> u8 {
        self[self.rwt(row)][self.cwl(col)] as u8
            + self[self.rwt(row)][col] as u8
            + self[self.rwt(row)][self.cwr(col)] as u8
            + self[row][self.cwl(col)] as u8
            + self[row][self.cwr(col)] as u8
            + self[self.rwb(row)][self.cwl(col)] as u8
            + self[self.rwb(row)][col] as u8
            + self[self.rwb(row)][self.cwr(col)] as u8
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
                write!(f, "{}", self[row][col])?;
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

pub struct Game {
    main_board: Board,
    shadow_board: Board,
    shadow_board_active: bool,
    ruleset: crate::Rules,
}

impl Game {
    pub fn new(cols: usize, rows: usize, ruleset: crate::Rules) -> Self {
        Game {
            main_board: Board::new(cols, rows),
            shadow_board: Board::new(cols, rows),
            shadow_board_active: false,
            ruleset,
        }
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
                dest[row][col] = Game::compute_next_cell_status(src, row, col, &self.ruleset);
            }
        }

        self.shadow_board_active = !self.shadow_board_active;
    }

    fn compute_next_cell_status(
        board: &Board,
        row: usize,
        col: usize,
        ruleset: &crate::Rules,
    ) -> Cell {
        let cell = board[row][col];
        let neighbour_count = board.alive_neighbours(row, col);
        match cell {
            Cell::Alive => {
                if ruleset.get_surviverule().contains(&neighbour_count) {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }
            Cell::Dead => {
                if ruleset.get_birthrule().contains(&neighbour_count) {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_active_board())
    }
}

impl std::ops::Index<usize> for Game {
    type Output = [Cell];
    fn index(&self, index: usize) -> &Self::Output {
        &self.get_active_board()[index]
    }
}

impl std::ops::IndexMut<usize> for Game {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.get_active_board_mut()[index]
    }
}
