use std::{
    fmt::{Debug, Display},
    ops::{Add, Index, IndexMut, Mul, Sub},
};

use crate::{Dimension, Get, GetMut};

use super::{
    ColIter, ColIterMut, MatIndex, MatrixColumn, MatrixColumnMut, MatrixRow, MatrixRowMut, RowIter,
    RowIterMut,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    dimension: Dimension,
    content: MatrixContent<T>,
}

impl<T> Matrix<T> {
    pub fn new(dimension: Dimension) -> Self
    where
        T: Default + Clone,
    {
        let size = dimension.len();
        Self {
            dimension,
            content: MatrixContent::new(size),
        }
    }

    pub fn with_init(dimension: Dimension, init: T) -> Self
    where
        T: Clone,
    {
        Self {
            dimension,
            content: MatrixContent::new_with_init(dimension.len(), init),
        }
    }

    pub fn size(&self) -> usize {
        self.dimension.len()
    }

    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    pub fn width(&self) -> isize {
        self.dimension().columns
    }

    pub fn height(&self) -> isize {
        self.dimension().rows
    }

    pub fn rows<'a>(&'a self) -> RowIter<'a, T> {
        RowIter { mat: self, pos: 0 }
    }

    pub fn columns<'a>(&'a self) -> ColIter<'a, T> {
        ColIter { mat: self, pos: 0 }
    }

    pub fn rows_mut<'a>(&'a mut self) -> RowIterMut<'a, T> {
        RowIterMut { mat: self, pos: 0 }
    }

    pub fn columns_mut<'a>(&'a mut self) -> ColIterMut<'a, T> {
        ColIterMut { mat: self, pos: 0 }
    }

    pub(super) fn buffer_mut(&mut self) -> &mut [T] {
        &mut self.content.values
    }

    pub(super) fn buffer(&self) -> &[T] {
        &self.content.values
    }
}

impl<'a, T: 'a> Get<'a, super::RowIndex> for Matrix<T> {
    type Output = MatrixRow<'a, T>;

    fn get(&'a self, index: super::RowIndex) -> Self::Output {
        MatrixRow {
            mat: self,
            row: index.val(),
        }
    }
}

impl<'a, T: 'a> GetMut<'a, super::RowIndex> for Matrix<T> {
    type Output = MatrixRowMut<'a, T>;

    fn get_mut(&'a mut self, index: super::RowIndex) -> Self::Output {
        MatrixRowMut {
            mat: self,
            row: index.val(),
        }
    }
}

impl<'a, T: 'a> Get<'a, super::ColIndex> for Matrix<T> {
    type Output = MatrixColumn<'a, T>;

    fn get(&'a self, index: super::ColIndex) -> Self::Output {
        MatrixColumn {
            mat: self,
            col: index.val(),
        }
    }
}

impl<'a, T: 'a> GetMut<'a, super::ColIndex> for Matrix<T> {
    type Output = MatrixColumnMut<'a, T>;

    fn get_mut(&'a mut self, index: super::ColIndex) -> Self::Output {
        MatrixColumnMut {
            mat: self,
            col: index.val(),
        }
    }
}

impl<T> Index<super::MatIndex> for Matrix<T> {
    type Output = T;

    fn index(&self, index: super::MatIndex) -> &Self::Output {
        let (row, col) = self.dimension.positate(index);
        let index = col as usize + (row * self.dimension.columns) as usize;

        &self.content.values[index]
    }
}

impl<T> IndexMut<super::MatIndex> for Matrix<T> {
    fn index_mut(&mut self, index: super::MatIndex) -> &mut Self::Output {
        let (row, col) = self.dimension.positate(index);
        let index = col as usize + (row * self.dimension.columns) as usize;

        &mut self.content.values[index]
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content_str = format!("{}", self.content.display(self.dimension));
        write!(f, "[\n{}\n]", content_str)
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content_str = format!("{:?}", self.content.debug(self.dimension));
        write!(f, "[\n{}\n]", content_str)
    }
}

impl<T: Clone + Add<T, Output = T>> Add<&Matrix<T>> for &Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.dimension() != rhs.dimension() {
            return None;
        }
        let mut res = self.clone();

        for r in 0..self.dimension().rows {
            for c in 0..self.dimension().columns {
                let index = MatIndex::new(r, c);
                res[index] = self[index].clone() + rhs[index].clone();
            }
        }

        Some(res)
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        todo!()
    }
}

impl<T: Clone + Mul<T, Output = T>> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.rows_mut()
            .for_each(|v| v.into_iter().for_each(|v| *v = v.clone() * rhs.clone()));

        self
    }
}

impl<T> Sub<Matrix<T>> for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixContent<T> {
    values: Box<[T]>,
}

impl<T> MatrixContent<T> {
    pub fn new(size: usize) -> Self
    where
        T: Default + Clone,
    {
        let vec = vec![T::default(); size];
        Self {
            values: vec.into_boxed_slice(),
        }
    }

    pub fn new_with_init(size: usize, init: T) -> Self
    where
        T: Clone,
    {
        let vec = vec![init; size];
        Self {
            values: vec.into_boxed_slice(),
        }
    }

    pub(crate) fn display(&self, dimension: Dimension) -> String
    where
        T: Display,
    {
        let mut output = String::new();
        for row in 0..dimension.rows {
            for col in 0..dimension.columns {
                let index = row * dimension.columns + col;
                let value = &self.values[index as usize];
                output.push_str(&format!("{} ", value));
            }
            output.push('\n');
        }
        output.pop();
        output
    }

    pub(crate) fn debug(&self, dimension: Dimension) -> String
    where
        T: Debug,
    {
        let mut output = String::new();
        for row in 0..dimension.rows {
            for col in 0..dimension.columns {
                let index = row * dimension.columns + col;
                let value = &self.values[index as usize];
                output.push_str(&format!("{:?} ", value));
            }
            output.push('\n');
        }
        output.pop();
        output
    }
}

#[derive(Clone)]
pub struct GenericMatrix(Matrix<i32>);

impl Add<Self> for &GenericMatrix {
    type Output = Option<GenericMatrix>;

    fn add(self, rhs: Self) -> Self::Output {
        match &self.0 + &rhs.0 {
            Some(v) => Some(GenericMatrix(v)),
            None => return None,
        }
    }
}

impl GenericMatrix {
    pub fn new(rows: isize, columns: isize) -> Self {
        Self(Matrix::new(Dimension::new(rows, columns)))
    }
    pub fn with(dimension: Dimension) -> Self {
        Self(Matrix::new(dimension))
    }

    pub fn with_init(dimension: Dimension, init: i32) -> Self {
        Self(Matrix::with_init(dimension, init))
    }
}

impl std::fmt::Debug for GenericMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for GenericMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::ops::Deref for GenericMatrix {
    type Target = Matrix<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GenericMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
