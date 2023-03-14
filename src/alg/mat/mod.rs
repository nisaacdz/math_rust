mod matrix;

pub(crate) mod gen;
mod matrixrow;
mod matrixcol;

use std::{ptr::NonNull, marker::PhantomData};

pub use matrixrow::*;
pub use matrixcol::*;

pub use gen::GenericMatrix;
pub use matrix::Matrix;

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
