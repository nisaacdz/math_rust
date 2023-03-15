#[macro_export]
macro_rules! matrix {
    [($rows:expr, $cols:expr), $($elem:expr),*] => {
        {
            use std::convert::TryInto;
            use math_rust::{Dimension, alg::matrices::{Matrix, MatrixContent}};

            let dim = Dimension::new($rows.try_into().unwrap(), $cols.try_into().unwrap());
            let content_vec = vec![$($elem),*];
            let content = MatrixContent::new_unchecked(content_vec);
            Matrix::with_content(dim, content)
        }
    }
}
