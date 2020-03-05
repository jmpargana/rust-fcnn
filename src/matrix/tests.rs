use super::*;

#[test]
fn constructors() {
    let _matrix_ints: Matrix<i32> = Matrix::new(10, 10);
    let _matrix_floats: Matrix<f64> = Matrix::new(2, 2);
    let _matrix_chars: Matrix<char> = Matrix::new(1, 40);
    let _matrix_bools: Matrix<bool> = Matrix::new(15, 1);
}
