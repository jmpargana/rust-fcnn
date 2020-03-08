// mod matrix;

// use matrix::Matrix;
use rand::Rng;

use ndarray::{arr1, arr2, Array1, Array2, Axis};




const ACT_FUNCTIONS: [&str; 6] = ["sigmoid", "tanh", "relu", "lrelu", "prelu", "softmax"];




/// Layer of a neural network
pub struct Layer {
    /// contains an activation function
    /// 
    /// in this case it is represented with a string which
    /// indicates which function to use from a list
    /// later it will indicate a HashMap or Tuple from string
    /// and function pointer
    activation: String,

    /// matrix of weights for each neutron in the layer
    weights: Array2<f64>,

    /// will save output value which gets used as input from next layer
    output: Array1<f64>,

    /// saves the sum vector which is needed to perform backpropagation
    sum_zs: Array1<f64>,
}




impl Layer {
    /// Creates an instance of a layer
    ///
    /// It contains a matrix of weights and empty vectors
    /// which get used in the forward and backpropagation
    ///
    /// # Arguments
    ///
    /// * `in_size` - size of input vector
    /// * `out_size` - size of output vector
    /// * `activation` - string referring to a predefined activation 
    /// function pointer
    ///
    /// # Example
    ///
    /// ```
    /// let layer = Layer::new(10, 20, "relu".to_string());
    /// ```
    ///
    /// # Panics
    ///
    /// if activation functino supplied was not defined in list
    pub fn new(in_size: usize, out_size: usize, activation: String) -> Layer {
        assert!(ACT_FUNCTIONS.contains(&&activation[..]));

        let mut rng = rand::thread_rng();

        let mut weights = Array2::zeros((in_size, out_size));

        for mut row in weights.axis_iter_mut(Axis(0)) {
            row.fill(rng.gen_range(0., 1.));
        }

        Layer {
            activation,
            weights,
            output: Array1::<f64>::zeros(out_size),
            sum_zs: Array1::<f64>::zeros(out_size),
        }
    }

    pub fn feed_forward(input: &Array2<f64>) {}

    pub fn gradients(input: &Array2<f64>) {}

    pub fn update_weights(input: &Array2<f64>) {}
}
