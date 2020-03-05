mod matrix;

use matrix::Matrix;

// Just not to get warnings for now
fn main() {
    let matrix: Matrix<f64> = Matrix::new(10, 10);
    assert!(matrix.rows() == 10 && matrix.cols() == 10);


    let transposed = matrix.transpose();
    
    let mut data = vec![vec![]];
    for col in 0..10 {
        data.push(matrix.get_col(col).unwrap().cloned().collect::<Vec<f64>>().clone());
    }

    let mat = Matrix::from(data);

    println!("{:?}", mat);
    println!("{:?}", transposed);
    println!("Hello, world!");
}
