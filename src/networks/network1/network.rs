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
pub struct Network1 {
    pub layers: Vec<Layer>,
    pub activation_fn: ActivationFunction,
    pub output_activation_fn: ActivationFunction,
}

impl BaseNetwork for Network1 {
    fn title(&self) -> String {
        "Network1".to_string()
    }
    fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        activation_fn: ActivationFunction,
        final_layer_activation_fn: ActivationFunction,
    ) -> Network1 {
        Network1 {
            layers: {
                let mut layers: Vec<Layer> = Vec::new();
                layers.push(Layer::new(internal_nodes, input_nodes));
                for _ in 0..(internal_layers - 1) {
                    layers.push(Layer::new(internal_nodes, internal_nodes));
                }
                layers.push(Layer::new(output_nodes, internal_nodes));
                layers
            },
            activation_fn,
            output_activation_fn: final_layer_activation_fn,
        }
    }
    fn replace_self(&mut self, other: &mut Self) {
        self.layers = other.layers.clone();
    }
    fn internel_layers(&self) -> usize {
        self.layers.len() - 1
    }
    fn internal_nodes(&self) -> usize {
        if self.layers.len() < 2 {
            return 0;
        }
        self.layers[0].nodes.len()
    }
    fn rand_weights(&mut self, rate: f64) {
        // println!("[network.rd] applying weights at rate: {}", rate);
        self.layers.iter_mut().for_each(|x| x.rand_weights(rate));
    }
    fn run(&mut self, initial_inputs: Vec<f64>) -> Vec<f64> {
        let _ = self.layers[0].run(&initial_inputs);
        for i in 1..self.layers.len() {
            let inputs = &self.layers[i - 1]
                .nodes
                .iter()
                .map(|x| self.activation_fn.forward(x.value))
                .collect::<Vec<f64>>();
            let _ = self.layers[i].run(inputs);
        }
        self.layers
            .last()
            .unwrap()
            .nodes
            .iter()
            .map(|x| self.output_activation_fn.forward(x.value))
            .collect::<Vec<f64>>()
    }

    fn revert(&mut self) {
        self.layers.iter_mut().for_each(|x| x.revert());
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

    fn activation_fn(&self) -> &ActivationFunction {
        &self.activation_fn
    }

    fn final_layer_activation_fn(&self) -> &ActivationFunction {
        &self.output_activation_fn
    }
}
