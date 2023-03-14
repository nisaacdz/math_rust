use std::ops::{Index, IndexMut};

use super::{ColIndex, MatIndex, Matrix};

#[derive(Debug, Clone)]
pub struct MatrixRow<'a, T> {
    pub(super) mat: &'a Matrix<T>,
    pub(super) row: isize,
}

pub struct MatrixRowMut<'a, T> {
    pub(super) mat: &'a mut Matrix<T>,
    pub(super) row: isize,
}

pub struct MatrixRowIter<'a, T> {
    mat: &'a Matrix<T>,
    index: MatIndex,
}

impl<'a, T> MatrixRow<'a, T> {
    pub(crate) fn positate(&self, index: isize) -> isize {
        if index < 0 {
            self.mat.cols() + index
        } else {
            index
        }
    }
}

impl<'a, T> MatrixRowMut<'a, T> {
    pub(crate) fn positate(&self, index: isize) -> isize {
        if index < 0 {
            self.mat.cols() + index
        } else {
            index
        }
    }
}

impl<'a, T> Iterator for MatrixRowIter<'a, T> {
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

impl<'a, T> IntoIterator for MatrixRow<'a, T> {
    type Item = &'a T;

    type IntoIter = MatrixRowIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixRowIter {
            index: MatIndex::new(0, self.row),
            mat: self.mat,
        }
    }
}

impl<'a, T> IntoIterator for MatrixRowMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = super::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.row * self.mat.cols();
        let end = start + self.mat.cols() - 1;
        unsafe { super::IterMut::new(start as usize, end as usize, self.mat.buffer_mut(), 1) }
    }
}

impl<'a, T> Index<isize> for MatrixRow<'a, T> {
    type Output = T;

    fn index(&self, mut index: isize) -> &Self::Output {
        index = self.positate(index);
        let p = index + (self.mat.cols() * self.row);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> Index<isize> for MatrixRowMut<'a, T> {
    type Output = T;

    fn index(&self, mut index: isize) -> &Self::Output {
        index = self.positate(index);
        let p = index + (self.mat.cols() * self.row);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> IndexMut<isize> for MatrixRowMut<'a, T> {
    fn index_mut(&mut self, mut index: isize) -> &mut Self::Output {
        index = self.positate(index);
        let p = index + (self.mat.cols() * self.row);
        &mut self.mat.buffer_mut()[p as usize]
    }
}

impl<'a, T> Index<ColIndex> for MatrixRow<'a, T> {
    type Output = T;

    fn index(&self, index: ColIndex) -> &Self::Output {
        let index = self.positate(index.col);
        let p = index + (self.mat.cols() * self.row);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> Index<ColIndex> for MatrixRowMut<'a, T> {
    type Output = T;

    fn index(&self, index: ColIndex) -> &Self::Output {
        let index = self.positate(index.col);
        let p = index + (self.mat.cols() * self.row);
        &self.mat.buffer()[p as usize]
    }
}

impl<'a, T> IndexMut<ColIndex> for MatrixRowMut<'a, T> {
    fn index_mut(&mut self, index: ColIndex) -> &mut Self::Output {
        let index = self.positate(index.col);
        let p = index + (self.mat.cols() * self.row);
        &mut self.mat.buffer_mut()[p as usize]
    }
}
