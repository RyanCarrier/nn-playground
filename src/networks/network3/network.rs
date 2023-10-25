use crate::traits::{generic_test_case::BatchResult, network_traits::BaseNetwork};

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
        activation: Option<fn(f64) -> f64>,
    ) -> Network3 {
        let output_fn = match activation {
            Some(x) => x,
            None => |x: f64| x.max(0.0),
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
        error_fn: Option<fn(&Vec<f64>, &Vec<f64>) -> Vec<f64>>,
    ) -> Result<BatchResult, String> {
        //cmon guys back propogation is simple 8), just copy it from chatgpt or smth
        let de_dy = |y: &Vec<f64>, t: &Vec<f64>| {
            y.iter()
                .zip(t.iter())
                .map(|(y, t)| y - t)
                .collect::<Vec<f64>>()
        };
        let layers = self.layers.len();
        // let input_length = self.layers[0].bias.len();
        // let output_length = self.layers[layers - 1].bias.len();
        let mut total_weight_gradients: Vec<Vec<Vec<f64>>> = self
            .layers
            .iter()
            .map(|x| x.weights.iter().map(|w| vec![0.0; w.len()]).collect())
            .collect();
        let mut total_bias_gradients: Vec<Vec<f64>> = self
            .layers
            .iter()
            .map(|x| vec![0.0; x.bias.len()])
            .collect();

        for case in test_cases {
            let input = case.get_input();
            let layer_results = self.run_by_steps(&input);
            let mut layer_result_gradients: Vec<Vec<f64>> =
                layer_results.iter().map(|x| vec![0.0; x.len()]).collect();
            //output layer grads (dE/dA^L)
            layer_result_gradients[layers - 1] = de_dy(&layer_results[layers - 1], &case.output);
            // println!("layer results: {:?}", layer_results);
            // println!("last layer grads: {:?}", layer_result_gradients);
            //figure out A^l gradients (dE/dA^l)
            for layer_index in (0..(layers - 1)).rev() {
                let layer_len = layer_result_gradients[layer_index].len();
                let deeper_layer_grad = layer_result_gradients[layer_index + 1].clone();
                for j in 0..layer_len {
                    let mut temp_grad = 0.0;
                    //next sum up all the contribute to this partial
                    for k in 0..deeper_layer_grad.len() {
                        //this is the dA[l+1]/dO[l+1]
                        // add all layer weights that pass activation?
                        if layer_results[layer_index + 1][k] > 0.0 {
                            //i think this might be incorrect, like we shoudl be adding all weights
                            //regardless, but only including per node section that is active
                            //DE/DA[l+1]
                            //dO[l+1]/dw[l]
                            temp_grad +=
                                self.layers[layer_index + 1].weights[k][j] * deeper_layer_grad[k]
                        }
                    }
                    layer_result_gradients[layer_index][j] = temp_grad;
                }
            }
            // println!("last layer grads(post): {:?}", layer_result_gradients);

            //update the total weight gradients, these are not reliant on each other, so all
            // we need is the Error wrt activations
            // we take dE/dA^l, with dA^l/dw^l_ij
            // we have dE/dA^l, layer_result_gradients
            // dA^l/dw^l_ij we get from dA^l/dO^l * dO^l/dw^l_ij
            // dA^l/dO^l is just 1>0 or 0
            // dO^l/dw^l_ij is activation of pref layer (a^(l-1)_i) = layer_results[i], or initial
            // inputs for layer 0
            for l in 0..layers {
                let prev_layer_activations = if l == 0 {
                    &input
                } else {
                    &layer_results[l - 1]
                };
                for j in 0..layer_results[l].len() {
                    if layer_results[l][j] > 0.0 {
                        total_bias_gradients[l][j] += layer_result_gradients[l][j];
                    }
                    for i in 0..prev_layer_activations.len() {
                        if prev_layer_activations[i] > 0.0 {
                            total_weight_gradients[l][j][i] +=
                                layer_result_gradients[l][j] * prev_layer_activations[i];
                        }
                    }
                }
            }
        }
        println!("total total_bias_gradients {:?}", total_bias_gradients);
        total_weight_gradients
            .iter()
            .for_each(|x| println!("total_weight_gradients: {:?}", x));
        let test_cases_len = test_cases.len() as f64;
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Pre  Weights: {:?}", x.weights));
        let my_rate = 0.2;
        for l in 0..layers {
            for j in 0..total_bias_gradients[l].len() {
                self.layers[l].bias[j] -=
                    0.1 * my_rate * (total_bias_gradients[l][j] / test_cases_len);
            }
            for j in 0..total_weight_gradients[l].len() {
                for i in 0..total_weight_gradients[l][j].len() {
                    self.layers[l].weights[j][i] -=
                        my_rate * (total_weight_gradients[l][j][i] / test_cases_len);
                }
            }
        }
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Post Weights: {:?}", x.weights));
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Post Biases: {:?}", x.bias));
        self.test_all(test_cases, error_fn)
    }
}
impl Network3 {
    // pub fn step(&self,layer:usize, inputs: Vec<f64>) -> Vec<f64> {
    //     self.layers[layer].run(inputs)
    // }
    pub fn run_by_steps(&self, initial_inputs: &Vec<f64>) -> Vec<Vec<f64>> {
        let mut layer_results: Vec<Vec<f64>> = vec![Vec::new(); self.layers.len()];
        layer_results[0] = self.layers[0].run(initial_inputs.clone());
        for i in 1..self.layers.len() {
            layer_results[i] = self.layers[i].run(layer_results[i - 1].clone());
        }
        layer_results
    }
}
