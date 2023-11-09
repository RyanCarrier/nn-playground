use rand::{thread_rng, Rng};

use crate::networks::activation_functions::ActivationFunction;

#[derive(Clone)]
pub struct Layer {
    //can we make compile time sized array?
    // pub input_size: usize,
    // pub output_size: usize,
    // f64x4
    pub weights: Vec<Vec<f64>>,
    old_weights: Vec<Vec<f64>>,
    // pub nodes: [[f64;input_size];output_size]],
    pub bias: Vec<f64>,
    old_bias: Vec<f64>,
    activation_fn: ActivationFunction,
}
impl Layer {
    pub fn new(input_size: usize, output_size: usize, activation_fn: ActivationFunction) -> Layer {
        Layer {
            weights: vec![vec![0.0; input_size]; output_size],
            old_weights: vec![vec![0.0; input_size]; output_size],
            bias: vec![0.0; output_size],
            old_bias: vec![0.0; output_size],
            activation_fn,
        }
    }
    pub fn rand_weights(&mut self, rate: f64) {
        let mut rng = thread_rng();
        let mut rand_rate = || rate * rng.gen_range(-1.0..1.0);
        self.old_weights = self.weights.clone();
        self.weights
            .iter_mut()
            .for_each(|x| x.iter_mut().for_each(|y| *y += rand_rate()));
        self.old_bias = self.bias.clone();
        self.bias.iter_mut().for_each(|x| *x += 0.1 * rand_rate());
    }
    pub fn run(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.weights
            .iter()
            .map(|paths| {
                paths
                    .iter()
                    .zip(inputs.iter())
                    .fold(0.0, |p, (x, y)| p + x * y)
            })
            .zip(self.bias.iter())
            .map(|(x, y)| self.activation_fn.forward(x + y))
            .collect()
    }

    pub fn revert(&mut self) {
        self.weights = self.old_weights.clone();
        self.bias = self.old_bias.clone();
    }
}
