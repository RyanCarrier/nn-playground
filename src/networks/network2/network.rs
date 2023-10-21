use crate::traits::network_traits::BaseNetwork;

use super::layer::Layer;

//these really don't need to be structs but they probably will need to be later?
#[derive(Clone)]
pub struct Network2 {
    pub layers: Vec<Layer>,
}

impl BaseNetwork for Network2 {
    fn title(&self) -> String {
        "Network2 (Vec<f64>)".to_string()
    }
    fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        output_fn: Option<fn(f64) -> f64>,
    ) -> Network2 {
        let output_fn = match output_fn {
            Some(x) => x,
            None => |x: f64| x.min(1.0).max(0.0),
            // None => |x: f64| x.max(0.0),
            // None => |x| x,
        };
        Network2 {
            layers: {
                let mut layers: Vec<Layer> = Vec::new();
                layers.push(Layer::new(input_nodes, internal_nodes, output_fn));
                for _ in 0..(internal_layers - 1) {
                    layers.push(Layer::new(internal_nodes, internal_nodes, output_fn));
                }
                layers.push(Layer::new(internal_nodes, output_nodes, output_fn));
                layers
            },
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

    fn revert(&mut self) {
        self.layers.iter_mut().for_each(|x| x.revert());
    }

    #[allow(unused_variables)]
    fn learn_from_testcases<I, O>(
        &mut self,
        test_cases: &Vec<crate::traits::generic_test_case::GenericTestCase<I, O>>,
        rate: f64,
    ) {
        todo!()
    }
}
