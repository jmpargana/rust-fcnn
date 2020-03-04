mod layer;
mod matrix;

use layer::Layer;
use matrix::Matrix;

use rand::Rng;


pub struct MultiLayerPerceptron {
    layers: Vec<Layer>
}

impl MultiLayerPerceptron {
    pub fn new() -> MultiLayerPerceptron {

    }

    pub fn forward_prop(input: Matrix<f64>) {

    }

    pub fn back_prop(expected: Matrix<f64>, loss: Matrix<f64>) {

    }
}
