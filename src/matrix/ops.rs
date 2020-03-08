// use rayon::prelude::*;
use std::marker::Send;

use super::Matrix;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

macro_rules! impl_op_basic {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T: $trait<Output = T>> $trait for Matrix<T> {
            type Output = Matrix<T>;

            fn $func(self, rhs: Self) -> Self::Output {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                Matrix {
                    rows: self.rows,
                    cols: self.cols,
                    data: self
                        .into_iter()
                        .zip(rhs.into_iter())
                        .map(|(a, b)| a $op b)
                        .collect(),
                }
            }
        }

        impl<'a: 'b, 'b, T> $trait for &'a Matrix<T>
        where
            &'a T: $trait<&'b T, Output = T>,
        {
            type Output = Matrix<T>;

            fn $func(self, rhs: &'b Matrix<T>) -> Self::Output {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                Matrix {
                    rows: self.rows,
                    cols: self.cols,
                    data: self
                        .iter()
                        .zip(rhs.iter())
                        .map(|(a, b)| a $op b)
                        .collect(),
                }
            }
        }
    }
}

macro_rules! impl_op_assign_basic {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T: $trait> $trait for Matrix<T> {
            fn $func(&mut self, rhs: Self) {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                self.data.iter_mut()
                    .zip(rhs.into_iter())
                    .for_each(|(a, b)| *a $op b);
            }
        }

        impl<'a, T: $trait<&'a T>> $trait<&'a Matrix<T>> for Matrix<T> {
            fn $func(&mut self, rhs: &'a Self) {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                self.data.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(a, b)| *a $op b);
            }
        }
    }
}

macro_rules! impl_op {
    ($trait:ident, $($more:ident),*) => {
        impl_op!($trait);
        impl_op!($($more),*);
    };

    (Add) => { impl_op_basic!(Add, add, +); };
    (Sub) => { impl_op_basic!(Sub, sub, -); };
    (AddAssign) => { impl_op_assign_basic!(AddAssign, add_assign, +=); };
    (SubAssign) => { impl_op_assign_basic!(SubAssign, sub_assign, -=); };
}

impl_op!(Add, AddAssign, Sub, SubAssign);

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<Output = T> + AddAssign + Copy + Send + Sync + Default + Clone,
{
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
                // let mut data = vec![T::default(); self.rows * rhs.cols];
                let mut data = Vec::with_capacity(self.rows * rhs.cols);

                // (0..self.rows).into_par_iter().map(|row| {
                //     (0..self.cols).into_par_iter().map(|col| {
                //         let mut iter = self.get_row(row).unwrap()
                //             .zip(rhs.get_col(col).unwrap());

                //         let (x, y) = iter.next().unwrap();
                //         let val = iter.fold(*x * *y, |mut acc, (a, b)| {
                //             acc += *a * *b;
                //             acc
                //         });
                //         data[col + row * rhs.cols] = val;
                //     });
                // });

                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        let mut iter = self.get_row(row).unwrap().zip(rhs.get_col(col).unwrap());

                        let (x, y) = iter.next().unwrap();
                        data.push(iter.fold(*x * *y, |mut acc, (a, b)| {
                            acc += *a * *b;
                            acc
                        }));
                    }
                }
                data
            },
        }
    }
}

impl<'a, 'b, T: AddAssign> Mul<&'b Matrix<T>> for &'a Matrix<T>
where
    &'a T: Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        assert!(self.cols == rhs.rows);

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * rhs.cols);

                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        let mut iter = self.get_row(row).unwrap().zip(rhs.get_col(col).unwrap());

                        let (x, y) = iter.next().unwrap();
                        data.push(iter.fold(x * y, |mut acc, (a, b)| {
                            acc += a * b;
                            acc
                        }));
                    }
                }
                data
            },
        }
    }
}
