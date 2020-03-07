use super::*;

#[test]
fn constructors() {
    let _matrix_ints: Matrix<i32> = Matrix::new(10, 10);
    let _matrix_floats: Matrix<f64> = Matrix::new(2, 2);
    let _matrix_chars: Matrix<char> = Matrix::new(1, 40);
    let _matrix_bools: Matrix<bool> = Matrix::new(15, 1);
}

#[test]
fn from_constructor() {
    let vec = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];

    let mat = Matrix::from(vec);

    assert_eq!(*mat.get(0, 1).unwrap(), 2.0);
    assert_eq!(*mat.get(1, 2).unwrap(), 3.0);
    assert_eq!(*mat.get(1, 0).unwrap(), 1.0);
    assert_eq!(*mat.get(1, 1).unwrap(), 2.0);
}

#[test]
#[should_panic]
fn invalid_cons() {
    let vec = vec![vec![1.0], vec![2.3, 3.0]];
    Matrix::from(vec);
}

#[test]
fn check_getter() {
    let vec = vec![vec![1], vec![2], vec![3]];
    let vec2 = vec![vec![1, 2, 3]];
    let mat = Matrix::from(vec);
    let mat2 = Matrix::from(vec2);

    assert_eq!(
        mat.get_row(0).unwrap().cloned().collect::<Vec<i32>>(),
        vec![1]
    );
    assert_eq!(
        mat.get_col(0).unwrap().cloned().collect::<Vec<i32>>(),
        vec![1, 2, 3]
    );
    assert_eq!(
        mat2.get_row(0).unwrap().cloned().collect::<Vec<i32>>(),
        vec![1, 2, 3]
    );
}

#[test]
fn check_trans() {
    let vec = vec![vec![1], vec![2], vec![3]];
    let vec2 = vec![vec![1, 2, 3]];

    let mat = Matrix::from(vec);
    let mat2 = Matrix::from(vec2);

    assert_eq!(mat.transpose(), mat2);
    assert_eq!(mat2.transpose(), mat);
}
