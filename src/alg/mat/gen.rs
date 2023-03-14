use std::ops::Add;

use crate::Dimension;

use super::Matrix;

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
