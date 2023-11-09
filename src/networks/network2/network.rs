use crate::{
    networks::activation_functions::ActivationFunction,
    traits::{
        generic_test_case::{BatchResult, GenericTestCase},
        network_traits::BaseNetwork,
    },
};

use super::layer::Layer;

//these really don't need to be structs but they probably will need to be later?
#[derive(Clone)]
pub struct Network2 {
    pub layers: Vec<Layer>,
    pub activation_fn: ActivationFunction,
    pub output_activation_fn: ActivationFunction,
}

impl BaseNetwork for Network2 {
    fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        activation_fn: ActivationFunction,
        output_activation_fn: ActivationFunction,
    ) -> Network2 {
        Network2 {
            layers: {
                let mut layers: Vec<Layer> = Vec::new();
                layers.push(Layer::new(input_nodes, internal_nodes, activation_fn));
                for _ in 0..(internal_layers - 1) {
                    layers.push(Layer::new(internal_nodes, internal_nodes, activation_fn));
                }
                layers.push(Layer::new(
                    internal_nodes,
                    output_nodes,
                    output_activation_fn,
                ));
                layers
            },
            activation_fn,
            output_activation_fn,
        }
    }
    fn title(&self) -> String {
        "Network2 (Vec<f64>)".to_string()
    }
    fn internel_layers(&self) -> usize {
        self.layers.len() - 1
    }
    fn internal_nodes(&self) -> usize {
        if self.layers.len() < 2 {
            return 0;
        }
        self.layers[0].bias.len()
    }
    fn rand_weights(&mut self, rate: f64) {
        self.layers.iter_mut().for_each(|x| x.rand_weights(rate));
    }
    fn run(&mut self, initial_inputs: Vec<f64>) -> Vec<f64> {
        self.layers
            .iter()
            .fold(initial_inputs, |inputs, layer| layer.run(inputs))
    }
    #[allow(unused_variables)]
    fn learn_from_testcases<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
        rate: f64,
        error_fn: Option<fn(f64, f64) -> f64>,
        d_error_fn: Option<fn(f64, f64) -> f64>,
    ) -> Result<BatchResult, String> {
        let pre = self.test_all(test_cases, error_fn);
        self.rand_weights(rate);
        if pre.is_err() {
            return pre;
        }
        let post = self.test_all(test_cases, None);
        if post.is_err() {
            self.revert();
            return pre;
        }
        if post.clone().unwrap().error > pre.clone().unwrap().error {
            self.revert();
            return pre;
        }
        return post;
    }

    fn revert(&mut self) {
        self.layers.iter_mut().for_each(|x| x.revert());
    }

    fn replace_self(&mut self, other: &mut Self) {
        self.layers = other.layers.clone();
    }

    fn activation_fn(&self) -> &ActivationFunction {
        &self.activation_fn
    }

    fn final_layer_activation_fn(&self) -> &ActivationFunction {
        &self.output_activation_fn
    }
}
