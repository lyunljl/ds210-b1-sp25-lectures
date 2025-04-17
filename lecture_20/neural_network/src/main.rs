use ndarray::prelude::*;
use rand::Rng;

// Helper function to populate arrays with random values
// We could use ndarray-rand crate to do this, but it doesn't seem to work in a Jupyter notebook
fn populate_array(arr: &mut Array2<f32>, m: usize, n: usize) {
    let mut rng = rand::rng();
    for i in 0..m {
        for j in 0..n {
            arr[(i, j)] = rng.random_range(-1.0..1.0);
        }
    }
}

// ReLU activation function
fn relu(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|x| if x > 0.0 { x } else { 0.0 })
}

// Derivative of ReLU
fn relu_derivative(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|x| if x > 0.0 { 1.0 } else { 0.0 })
}

// Softmax function
fn softmax(x: &Array2<f32>) -> Array2<f32> {
    let exp_x = x.mapv(|x| x.exp());
    let sum_exp_x = exp_x.sum();
    exp_x / sum_exp_x
}

struct NeuralNetwork {
    input_size: usize,
    output_size: usize,
    weights1: Array2<f32>,
    biases1: Array2<f32>,
    weights2: Array2<f32>,
    biases2: Array2<f32>,
    learning_rate: f32,
}

impl NeuralNetwork {
    /// Create a shallow neural network with one hidden layer
    /// 
    /// # Arguments
    /// 
    /// * `input_size` - The number of input neurons
    /// * `hidden_size` - The number of neurons in the hidden layer
    /// * `output_size` - The number of output neurons
    /// * `learning_rate` - The learning rate for the neural network
    fn new(input_size: usize, hidden_size: usize, output_size: usize, learning_rate: f32) -> Self {
        let mut weights1 = Array2::zeros((hidden_size, input_size));
        let mut weights2 = Array2::zeros((output_size, hidden_size));

        // Initialize weights with random values
        populate_array(&mut weights1, hidden_size, input_size);
        populate_array(&mut weights2, output_size, hidden_size);

        let biases1 = Array2::zeros((hidden_size, 1));
        let biases2 = Array2::zeros((output_size, 1));

        NeuralNetwork {
            input_size,
            output_size,
            weights1,
            biases1,
            weights2,
            biases2,
            learning_rate,
        }
    }

    fn forward(&self, input: &Array2<f32>) -> (Array2<f32>, Array2<f32>, Array2<f32>) {
        // First layer
        let pre_activation1 = self.weights1.dot(input) + &self.biases1;
        let hidden = relu(&pre_activation1);

        // Output layer
        let pre_activation2 = self.weights2.dot(&hidden) + &self.biases2;
        let output = softmax(&pre_activation2);

        (hidden, pre_activation2, output)
    }

    fn backward(
        &mut self,
        input: &Array2<f32>,
        hidden: &Array2<f32>,
        pre_activation2: &Array2<f32>,
        output: &Array2<f32>,
        target: &Array2<f32>,
    ) {
        let batch_size = input.shape()[1] as f32;

        // Calculate gradients for output layer
        let output_error = output - target;
        
        // Gradients for weights2 and biases2
        let d_weights2 = output_error.dot(&hidden.t()) / batch_size;
        let d_biases2 = &output_error.sum_axis(Axis(1)).insert_axis(Axis(1)) / batch_size;

        // Backpropagate error to hidden layer
        let hidden_error = self.weights2.t().dot(&output_error);
        let hidden_gradient = &hidden_error * &relu_derivative(hidden);

        // Gradients for weights1 and biases1
        let d_weights1 = hidden_gradient.dot(&input.t()) / batch_size;
        let d_biases1 = &hidden_gradient.sum_axis(Axis(1)).insert_axis(Axis(1)) / batch_size;

        // Update weights and biases using gradient descent
        self.weights2 = &self.weights2 - &(d_weights2 * self.learning_rate);
        self.biases2 = &self.biases2 - &(d_biases2 * self.learning_rate);
        self.weights1 = &self.weights1 - &(d_weights1 * self.learning_rate);
        self.biases1 = &self.biases1 - &(d_biases1 * self.learning_rate);
    }

    fn train(&mut self, input: &Array2<f32>, target: &Array2<f32>) -> f32 {
        // Forward pass
        let (hidden, pre_activation2, output) = self.forward(input);

        // Calculate loss (cross-entropy)
        let epsilon = 1e-15;
        let loss = -target * &output.mapv(|x| (x + epsilon).ln());
        let loss = loss.sum() / (input.shape()[1] as f32);

        // Backward pass
        self.backward(input, &hidden, &pre_activation2, &output, target);

        loss
    }
}

fn main() {
    // Create a neural network
    let mut nn = NeuralNetwork::new(6, 6, 4, 0.01);

    // Create sample input
    let mut input = Array2::zeros((nn.input_size, 1));
    populate_array(&mut input, nn.input_size, 1);

    // Create sample target
    let mut target = Array2::zeros((nn.output_size, 1));
    populate_array(&mut target, nn.output_size, 1);
    
    // Calculate initial cross entropy loss before training
    let (_, _, initial_output) = nn.forward(&input);
    let epsilon = 1e-15;
    let initial_loss = -&target * &initial_output.mapv(|x| (x + epsilon).ln());
    let initial_loss = initial_loss.sum() / (input.shape()[1] as f32);
    println!("Initial loss before training: {}", initial_loss);

    
    // Train for one iteration
    let loss = nn.train(&input, &target);
    //println!("Loss: {}", loss);

    // Calculate loss after training
    let (_, _, final_output) = nn.forward(&input);
    let epsilon = 1e-15;
    let final_loss = -&target * &final_output.mapv(|x| (x + epsilon).ln());
    let final_loss = final_loss.sum() / (input.shape()[1] as f32);
    println!("loss after training: {}", final_loss);

}
