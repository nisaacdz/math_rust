mod cols;
mod macros;
mod matrix;
mod rows;

use std::{marker::PhantomData, ptr::NonNull};

pub use cols::*;
pub use matrix::*;
pub use rows::*;

pub use macros::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowIndex {
    pub row: isize,
}

impl RowIndex {
    pub fn new(row: isize) -> Self {
        Self { row }
    }
    pub fn val(&self) -> isize {
        self.row
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColIndex {
    pub col: isize,
}

impl ColIndex {
    pub fn new(col: isize) -> Self {
        Self { col }
    }
    pub fn val(&self) -> isize {
        self.col
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatIndex {
    pub row: isize,
    pub col: isize,
}

impl MatIndex {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn from(row: RowIndex, col: ColIndex) -> Self {
        Self {
            row: row.val(),
            col: col.val(),
        }
    }

    pub fn row(&self) -> isize {
        self.row
    }

    pub fn col(&self) -> isize {
        self.col
    }
}

pub struct RowIter<'a, T> {
    mat: &'a Matrix<T>,
    pos: isize,
}

impl<'a, T: 'a> Iterator for RowIter<'a, T> {
    type Item = MatrixRow<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.mat.height() {
            None
        } else {
            self.pos += 1;
            Some(MatrixRow {
                mat: self.mat,
                row: self.pos - 1,
            })
        }
    }
}

pub struct ColIter<'a, T> {
    mat: &'a Matrix<T>,
    pos: isize,
}

impl<'a, T: 'a> Iterator for ColIter<'a, T> {
    type Item = MatrixColumn<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.mat.width() {
            None
        } else {
            self.pos += 1;
            Some(MatrixColumn {
                mat: self.mat,
                col: self.pos - 1,
            })
        }
    }
}

pub struct RowIterMut<'a, T> {
    mat: &'a mut Matrix<T>,
    pos: isize,
}

impl<'a, T> Iterator for RowIterMut<'a, T> {
    type Item = MatrixRowMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.mat.height() {
            None
        } else {
            todo!()
        }
    }
}

pub struct ColIterMut<'a, T> {
    mat: &'a mut Matrix<T>,
    pos: isize,
}

impl<'a, T> Iterator for ColIterMut<'a, T> {
    type Item = MatrixColumnMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.mat.height() {
            None
        } else {
            todo!()
        }
    }
}

/// IterMut

#[derive(Debug, Clone)]
pub struct IterMut<'a, T: 'a> {
    ptr: NonNull<T>,
    end: *mut T,
    step: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) unsafe fn new(start: usize, end: usize, slice: &'a mut [T], step: usize) -> Self {
        assert!(start <= end);
        assert!(step > 0);
        let ptr = slice.as_mut_ptr().add(start);
        let end = slice.as_mut_ptr().add(end).add(step);
        Self {
            ptr: NonNull::new_unchecked(ptr),
            end,
            step,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next_ptr = self.ptr.as_ptr().add(self.step);
            if next_ptr <= self.end {
                let curr_ptr = self.ptr.as_mut();
                self.ptr = NonNull::new_unchecked(next_ptr);
                Some(curr_ptr)
            } else {
                None
            }
        }
    }
}
