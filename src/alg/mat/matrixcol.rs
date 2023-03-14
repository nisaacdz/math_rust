use std::ops::{Index, IndexMut};

use super::{MatIndex, Matrix, RowIndex};

#[derive(Debug, Clone)]
pub struct MatrixColumn<'a, T> {
    pub(super) mat: &'a Matrix<T>,
    pub(super) col: isize,
}

pub struct MatrixColumnMut<'a, T> {
    pub(super) mat: &'a mut Matrix<T>,
    pub(super) col: isize,
}

impl<'a, T> MatrixColumn<'a, T> {
    pub(crate) fn positate(&self, index: isize) -> isize {
        if index < 0 {
            self.mat.rows() + index
        } else {
            index
        }
    }
}

impl<'a, T> MatrixColumnMut<'a, T> {
    pub(crate) fn positate(&self, index: isize) -> isize {
        if index < 0 {
            self.mat.rows() + index
        } else {
            index
        }
    }
}

pub struct MatrixColumnIter<'a, T> {
    mat: &'a Matrix<T>,
    index: MatIndex,
}

impl<'a, T> Iterator for MatrixColumnIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index.row == self.mat.dimension().rows {
            None
        } else {
            self.index.row += 1;
            let index = self.index.clone();
            Some(&self.mat[index])
        }
    }
}

impl<'a, T> IntoIterator for MatrixColumn<'a, T> {
    type Item = &'a T;

    type IntoIter = MatrixColumnIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixColumnIter {
            index: MatIndex::new(0, self.col),
            mat: self.mat,
        }
    }
}

impl<'a, T> IntoIterator for MatrixColumnMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = super::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.col;
        let end = self.col + self.mat.cols() * (self.mat.rows() - 1);
        let step = self.mat.cols();
        unsafe {
            super::IterMut::new(
                start as usize,
                end as usize,
                self.mat.buffer_mut(),
                step as usize,
            )
        }
    }
}

impl<'a, T> Index<isize> for MatrixColumn<'a, T> {
    type Output = T;

    fn index(&self, mut index: isize) -> &Self::Output {
        index = self.positate(index);
        let p = self.col + (self.mat.cols() * index);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> Index<isize> for MatrixColumnMut<'a, T> {
    type Output = T;

    fn index(&self, mut index: isize) -> &Self::Output {
        index = self.positate(index);
        let p = self.col + (self.mat.cols() * index);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> IndexMut<isize> for MatrixColumnMut<'a, T> {
    fn index_mut(&mut self, mut index: isize) -> &mut Self::Output {
        index = self.positate(index);
        let p = self.col + (self.mat.cols() * index);
        &mut self.mat.buffer_mut()[p as usize]
    }
}

impl<'a, T> Index<RowIndex> for MatrixColumn<'a, T> {
    type Output = T;

    fn index(&self, index: RowIndex) -> &Self::Output {
        let index = self.positate(index.row);
        let p = self.col + (self.mat.cols() * index);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> Index<RowIndex> for MatrixColumnMut<'a, T> {
    type Output = T;

    fn index(&self, index: RowIndex) -> &Self::Output {
        let index = self.positate(index.row);
        let p = self.col + (self.mat.cols() * index);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> IndexMut<RowIndex> for MatrixColumnMut<'a, T> {
    fn index_mut(&mut self, index: RowIndex) -> &mut Self::Output {
        let index = self.positate(index.row);
        let p = self.col + (self.mat.cols() * index);
        &mut self.mat.buffer_mut()[p as usize]
    }
}
