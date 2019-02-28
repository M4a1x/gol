#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Cell::Alive => "X",
            Cell::Dead => "."
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    data: Box<[Cell]>,
    cols: usize,
    rows: usize,
    iteration: u64
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Self {
        Board {
            data: vec![Cell::Dead; cols * rows].into_boxed_slice(),
            cols,
            rows,
            iteration: 0
        }
    }

    pub fn write_next_gen(&self, output: &mut Board) {
        for col in 0..self.cols {
            for row in 0..self.rows {
                let neighbours = self.alive_neighbours(row, col);
                output[row][col] = match self[row][col] {
                    Cell::Alive => {
                        if neighbours == 2 || neighbours == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    },
                    Cell::Dead => {
                        if neighbours == 3 {Cell::Alive} else {Cell::Dead}
                    }
                }
            }
        }
    }

    fn alive_neighbours(&self, row: usize, col: usize) -> u8 {
          self[self.rwt(row)][self.cwl(col)] as u8 + self[self.rwt(row)][col] as u8 + self[self.rwt(row)][self.cwr(col)] as u8
        + self[row][self.cwl(col)] as u8                                            + self[row][self.cwr(col)] as u8
        + self[self.rwb(row)][self.cwl(col)] as u8 + self[self.rwb(row)][col] as u8 + self[self.rwb(row)][self.cwr(col)] as u8
    }

    fn cwl(&self, col: usize) -> usize {
        if col == 0 {self.cols - 1} else {col - 1}
    }

    fn cwr(&self, col: usize) -> usize {
        if col == self.cols - 1 {0} else {col + 1}
    }

    fn rwt(&self, row: usize) -> usize {
        if row == 0 {self.rows - 1} else {row - 1}
    }

    fn rwb(&self, row: usize) -> usize {
        if row == self.rows - 1 {0} else {row + 1}
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self[row][col]).expect("Not written");
            }
            writeln!(f, "").expect("Not written");
        }
        Ok(())
    }
}

impl std::ops::Index<usize> for Board {
    type Output = [Cell];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols .. (index+1) * self.cols]
    }
}

impl std::ops::IndexMut<usize> for Board {
    fn index_mut(& mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols .. (index+1) * self.cols]
    }
}