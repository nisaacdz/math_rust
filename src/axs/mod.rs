use crate::alg::matrices::{ColIndex, RowIndex, MatIndex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dimension {
    pub rows: isize,
    pub columns: isize,
}

impl Dimension {
    pub fn new(rows: isize, columns: isize) -> Self {
        Self { rows, columns }
    }

    pub fn len(&self) -> usize {
        (self.columns * self.rows) as usize
    }

    pub fn positate_row(&self, RowIndex { row }: RowIndex) -> isize {
        if row < 0 {
            self.rows as isize + row
        } else {
            row
        }
    }

    pub fn positate_col(&self, ColIndex { col }: ColIndex) -> isize {
        if col < 0 {
            self.columns as isize + col
        } else {
            col
        }
    }

    pub fn positate(&self, MatIndex { row, col }: MatIndex) -> (isize, isize) {
        (
            if row < 0 {
                self.rows as isize + row
            } else {
                row
            },
            if col < 0 {
                self.columns as isize + col
            } else {
                col
            },
        )
    }
}
