use crate::traits::network_traits::BaseNetwork;

use super::layer::Layer;

//these really don't need to be structs but they probably will need to be later?
#[derive(Clone)]
pub struct Network3 {
    pub layers: Vec<Layer>,
}

impl BaseNetwork for Network3 {
    fn title(&self) -> String {
        "Network3 (Vec<f64>)".to_string()
    }
    fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        output_fn: Option<fn(f64) -> f64>,
    ) -> Network3 {
        let output_fn = match output_fn {
            Some(x) => x,
            None => |x: f64| x.min(1.0).max(0.0),
            // None => |x: f64| x.max(0.0),
            // None => |x| x,
        };
        Network3 {
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
    fn learn_from_testcases<I, O>(
        &mut self,
        test_cases: &Vec<crate::traits::generic_test_case::GenericTestCase<I, O>>,
        rate: f64,
    ) {
        let de_dy = |y: &Vec<f64>, t: &Vec<f64>| {
            y.iter()
                .zip(t.iter())
                .map(|(y, t)| y - t)
                .collect::<Vec<f64>>()
        };
        for case in test_cases {
            let layer_results = self.run_by_steps(case.get_input().clone());
            let mut layer_result_gradients: Vec<Vec<f64>> = layer_results.clone();
        }
    }
}
impl Network3 {
    // pub fn step(&self,layer:usize, inputs: Vec<f64>) -> Vec<f64> {
    //     self.layers[layer].run(inputs)
    // }
    pub fn run_by_steps(&self, initial_inputs: Vec<f64>) -> Vec<Vec<f64>> {
        let mut layer_results: Vec<Vec<f64>> = vec![Vec::new(); self.layers.len()];
        layer_results[0] = self.layers[0].run(initial_inputs.clone());
        for i in 1..self.layers.len() {
            layer_results[i] = self.layers[i].run(layer_results[i - 1].clone());
        }
        layer_results
    }
}
