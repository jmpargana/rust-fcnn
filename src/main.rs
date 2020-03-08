mod layer;
mod matrix;

use layer::Layer;
use matrix::Matrix;

// Just not to get warnings for now
fn main() {
    let layer = Layer::new(4, 4, "relu".to_string());

    let matrix: Matrix<f64> = Matrix::new(10, 10);
    assert!(matrix.rows() == 10 && matrix.cols() == 10);

    let transposed = matrix.transpose();

    let mut data = vec![vec![]];
    for col in 0..10 {
        data.push(
            matrix
                .get_col(col)
                .unwrap()
                .cloned()
                .collect::<Vec<f64>>()
                .clone(),
        );
    }

    let mat1 = vec![vec![1.0], vec![1.0]];
    let mat2 = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]];

    let mat = Matrix::from(mat2);

    println!("{:?}", mat);
    println!("{:?}", Matrix::from(mat1).transpose() * mat);
    println!("{:?}", transposed);
}
