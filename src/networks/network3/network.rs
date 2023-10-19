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

    fn back_prop_round<I, O>(
        &mut self,
        test_cases: &Vec<crate::traits::generic_test_case::GenericTestCase<I, O>>,
    ) {
        let mut de_dy = |x: f64, t: f64| x - t;
        for case in test_cases {
            let mut state = vec![];
        }
    }

    fn learn_from_results(&mut self, results: Vec<Vec<f64>>, expected: Vec<Vec<f64>>, rate: f64) {
        let mut difference: Vec<f64> = vec![0.0; results[0].len()];
        for i in 0..results.len() {
            for j in 0..results[i].len() {
                let diff = results[i][j] - expected[i][j];
                let sign = if (diff.is_sign_positive()) { 1.0 } else { -1.0 };
                difference[j] += sign * diff.powi(2);
            }
        }
        let rl = results.len() as f64;
        difference.iter_mut().for_each(|x| *x /= rl);
        for l in (self.layers.len() - 1)..=0 {
            difference = self.layers[l].learn_from_results(difference, rate);
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
}
