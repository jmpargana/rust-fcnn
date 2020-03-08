#[path="../layer/mod.rs"]
mod layer;


use layer::Layer;
use ndarray::Array1;



pub struct MultiLayerPerceptron {
    hidden_layers: Vec<Layer>,
    output_layer: Layer,
    batch_size: usize,
    epoch_size: usize,
}



impl MultiLayerPerceptron {
    pub fn new() -> MultiLayerPerceptron {
        // hard coded just for now
        MultiLayerPerceptron {
            hidden_layers: vec![],
            output_layer: Layer::new(4, 4, "relu".to_string()),
            batch_size: 10,
            epoch_size: 10,
        }        
    }

    pub fn forward_prop(input: Array1<f64>) {

    }

    pub fn back_prop(expected: Array1<f64>, loss: Array1<f64>) {

    }
}
