mod matrix;

use matrix::Matrix;
use rand::Rng;


const ACT_FUNCTIONS = ["sigmoid", "tanh", "relu", "lrelu", "prelu", "softmax"];


pub struct Layer {
    activation: &str,
    weights: Matrix<f64>,
    output: Matrix<f64>,
    sum_zs: Matrix<f64>,
}

impl Layer {
    pub fn new(in_size: usize, out_size: usize, activation: &str) -> Layer {
        assert!(ACT_FUNCTIONS.contains(activation));

        let weights = Matrix::from(
            vec![vec![rand::thread_rng().gen_range(0.0, 1.0); in_size]; out_size]);

        Layer {
            activation,
            weights,
            output: Matrix::new(1, out_size),
            sum_zs: Matrix::new(1, out_size),
        }
    }

    pub fn feed_forward(input: &Matrix<f64>) {

    }

    pub fn gradients(input: &Matrix<f64>) {

    }

    pub fn update_weights(input: &Matrix<f64>) {

    }
}

