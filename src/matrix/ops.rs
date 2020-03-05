use super::Matrix;
use std::ops::{
    Add,
    AddAssign,
    Mul,
    Sub,
    SubAssign,
}


impl<T> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    /// Std Ops Multiplication implementation for Matrix struct
    ///
    /// perform dot product for each combination of row and column
    /// and save in new instance
    ///
    /// # Arguments 
    ///
    /// * `lhs` - current matrix
    /// * `rhs` - other matrix
    ///
    /// # Example 
    ///
    /// ```
    ///
    /// ```
    ///
    /// # Panics
    ///
    /// If two matrices don't match
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.cols == rhs.rows);

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * rhs.cols);

                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        data.push(self.get_row(row).unwrap()
                            .zip(rhs.get_col(col).unwrap())
                            .map(|(a, b)| *a * *y).sum());
                    }
                }
                data
            },
        }
    }
}
