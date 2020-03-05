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

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.cols == rhs.rows);

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * rhs.cols);

                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        // dot product of row and col
                        let acc = self.get_row(row).unwrap()
                            .zip(rhs.get_col(col).unwrap())
                            .map(|(a, b)| *a * *y).sum();

                        data.push(acc);
                    }
                }

                data
            },
        }
    }
}
