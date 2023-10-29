use crate::traits::{
    generic_test_case::{BatchResult, GenericTestCase},
    network_traits::BaseNetwork,
};

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
        activation_fn: fn(f64) -> f64,
    ) -> Network3 {
        // Network3 {
        //     layers: {
        //         let mut layers: Vec<Layer> = Vec::new();
        //         layers.push(Layer::new(input_nodes, internal_nodes, output_fn));
        //         for _ in 0..(internal_layers - 1) {
        //             layers.push(Layer::new(internal_nodes, internal_nodes, output_fn));
        //         }
        //         layers.push(Layer::new(internal_nodes, output_nodes, output_fn));
        //         layers
        //     },
        // }
        Network3 {
            layers: {
                let mut layers: Vec<Layer> = Vec::new();
                layers.push(Layer::new(input_nodes, internal_nodes, activation_fn));
                for _ in 0..(internal_layers - 1) {
                    layers.push(Layer::new(internal_nodes, internal_nodes, activation_fn));
                }
                layers.push(Layer::new(internal_nodes, output_nodes, activation_fn));
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
        test_cases: &Vec<GenericTestCase<I, O>>,
        rate: f64,
        error_fn: Option<fn(f64, f64) -> f64>,
        d_error_fn: Option<fn(f64, f64) -> f64>,
        d_activation_fn: fn(f64) -> f64,
    ) -> Result<BatchResult, String> {
        let d_error_fn = match d_error_fn {
            Some(x) => x,
            None => |y: f64, t: f64| (y - t),
        };
        let rate = 0.2;
        for case in test_cases.iter() {
            let (layer_outputs, layer_activations) = self.run_by_steps(&case.get_input());
            let mut layer_gradient = layer_activations
                .last()
                .unwrap()
                .iter()
                .zip(case.output.iter())
                .map(|(y, t)| d_error_fn(*y, *t))
                .collect::<Vec<f64>>();
            for l in (0..(self.layers.len())).rev() {
                let next_layer_gradient = if l == 0 {
                    vec![0.0; 0]
                } else {
                    let mut temp_next_layer_grad: Vec<f64> = Vec::new();
                    for i in 0..self.layers[l].weights[0].len() {
                        let mut temp_grad = 0.0;
                        for j in 0..self.layers[l].weights.len() {
                            let da_do = d_activation_fn(layer_outputs[l][j]);
                            temp_grad += da_do * self.layers[l].weights[j][i] * layer_gradient[j];
                        }
                        temp_next_layer_grad.push(temp_grad);
                    }
                    temp_next_layer_grad
                };

                for j in 0..self.layers[l].weights.len() {
                    let da_do = d_activation_fn(layer_outputs[l][j]);
                    for i in 0..self.layers[l].weights[j].len() {
                        let prev_activations = if l == 0 {
                            case.get_input().clone()
                        } else {
                            layer_activations[l - 1].clone()
                        };
                        // let prev_outputs = if l == 0 {
                        //     case.get_input().clone()
                        // } else {
                        //     layer_outputs[l - 1].clone()
                        // };
                        self.layers[l].weights[j][i] -=
                            // rate * prev_activations[i] * layer_gradient[j];
                        rate * da_do * prev_activations[i] * layer_gradient[j];
                        // rate * da_do * prev_outputs[i] * layer_gradient[j];
                    }
                    self.layers[l].bias[j] -= rate * da_do * layer_gradient[j];
                }
                //probs don't need to clone but idc rn
                layer_gradient = next_layer_gradient.clone();
            }
        }
        self.test_all(test_cases, error_fn)
    }
}
impl Network3 {
    // pub fn step(&self,layer:usize, inputs: Vec<f64>) -> Vec<f64> {
    //     self.layers[layer].run(inputs)
    // }
    pub fn new_default(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
    ) -> Network3 {
        Network3::new(
            input_nodes,
            output_nodes,
            internal_nodes,
            internal_layers,
            Self::activation_fn,
        )
    }
    pub fn run_by_steps(&self, initial_inputs: &Vec<f64>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut layer_outputs: Vec<Vec<f64>> = vec![Vec::new(); self.layers.len()];
        let mut layer_activations: Vec<Vec<f64>> = vec![Vec::new(); self.layers.len()];
        layer_outputs[0] = self.layers[0].run_total(initial_inputs.clone());
        layer_activations[0] = self.layers[0].run_activate(layer_outputs[0].clone());
        for i in 1..self.layers.len() {
            layer_outputs[i] = self.layers[i].run(layer_activations[i - 1].clone());
            layer_activations[i] = self.layers[i].run(layer_outputs[i].clone());
        }
        (layer_outputs, layer_activations)
    }
    pub fn d_activation_fn(x: f64) -> f64 {
        let activation = Self::activation_fn(x);
        activation * (1.0 - activation)
    }
    pub fn activation_fn(x: f64) -> f64 {
        1.0 / (1.0 + f64::exp(-x))
    }

    #[allow(dead_code, unused_variables)]
    pub fn learn_from_testcases_first_attempt<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
        rate: f64,
        error_fn: Option<fn(f64, f64) -> f64>,
        d_error_fn: Option<fn(f64, f64) -> f64>,
        d_activation_fn: fn(f64) -> f64,
    ) -> Result<BatchResult, String> {
        let error_fn = match error_fn {
            Some(x) => x,
            None => |y: f64, t: f64| (t - y).powi(2) / 2.0,
        };
        let d_error_fn = match d_error_fn {
            Some(x) => x,
            None => |y: f64, t: f64| (y - t),
        };

        //cmon guys back propogation is simple 8), just copy it from chatgpt or smth
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
        let rate = 0.1;

        for case in test_cases {
            let input = case.get_input();
            let (layer_outputs, layer_activations) = self.run_by_steps(&input);
            let mut layer_result_gradients: Vec<Vec<f64>> = layer_activations
                .iter()
                .map(|x| vec![0.0; x.len()])
                .collect();
            //output layer gralayer_outputs,ds (dE/dA^L)
            layer_result_gradients[layers - 1] = layer_activations[layers - 1]
                .iter()
                .zip(case.output.iter())
                .map(|(y, t)| d_error_fn(*y, *t))
                .collect();
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
                        // if layer_results[layer_index + 1][k] > 0.0 {
                        //i think this might be incorrect, like we shoudl be adding all weights
                        //regardless, but only including per node section that is active
                        //DE/DA[l+1]
                        //dO[l+1]/dw[l]
                        temp_grad += d_activation_fn(layer_outputs[layer_index + 1][k])
                            * self.layers[layer_index + 1].weights[k][j]
                            * deeper_layer_grad[k]
                        // }
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
                    &layer_activations[l - 1]
                };
                for j in 0..layer_activations[l].len() {
                    // if layer_results[l][j] > 0.0 {
                    total_bias_gradients[l][j] +=
                        d_activation_fn(layer_outputs[l][j]) * layer_result_gradients[l][j];
                    // }
                    for i in 0..prev_layer_activations.len() {
                        if prev_layer_activations[i] > 0.0 {
                            total_weight_gradients[l][j][i] +=
                                layer_result_gradients[l][j] * prev_layer_activations[i];
                        }
                    }
                }
            }
            // println!(
            //     "input: {:?}, expected output:{:?}, got:{:?}",
            //     input,
            //     case.output,
            //     layer_results[layers - 1]
            // );
            // println!("layer_results: {:?}", layer_results);
        }
        // println!("total total_bias_gradients {:?}", total_bias_gradients);
        // total_weight_gradients
        //     .iter()
        //     .for_each(|x| println!("total_weight_gradients: {:?}", x));
        let test_cases_len = test_cases.len() as f64;
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Pre  Weights: {:?}", x.weights));
        for l in 0..layers {
            for j in 0..total_bias_gradients[l].len() {
                self.layers[l].bias[j] -=
                    0.1 * rate * (total_bias_gradients[l][j] / test_cases_len);
            }
            for j in 0..total_weight_gradients[l].len() {
                for i in 0..total_weight_gradients[l][j].len() {
                    self.layers[l].weights[j][i] -=
                        rate * (total_weight_gradients[l][j][i] / test_cases_len);
                }
            }
        }
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Post Weights: {:?}", x.weights));
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Post Biases: {:?}", x.bias));
        self.test_all(test_cases, Some(error_fn))
    }
    #[allow(dead_code, unused_variables)]
    fn learn_from_testcases_second_attempt<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
        rate: f64,
        error_fn: Option<fn(f64, f64) -> f64>,
        d_error_fn: Option<fn(f64, f64) -> f64>,
        d_activation_fn: fn(f64) -> f64,
    ) -> Result<BatchResult, String> {
        // attempt 2 at back propogation
        // main change is not consolidating all the gradients before applying
        // so every round we apply gradients and hopefully we can follow better paths to success
        // rather than following path to the lowest average error (per whole set, ie
        // always spitting a result that will be 'mostly correct' all the time or something like
        // that)
        // this is cause if done all at once we we want half results to be 0.0 and half to be 1.0
        // so consolidating made it so we want it to be 0.5 all the time... lol
        let d_error_fn = match d_error_fn {
            Some(x) => x,
            None => |y: f64, t: f64| (y - t),
        };
        let rate = 0.05;

        //cmon guys back propogation is simple 8), just copy it from chatgpt or smth
        let layers = self.layers.len();

        for case in test_cases {
            let input = case.get_input();
            let (layer_outputs, layer_activations) = self.run_by_steps(&input);
            let mut layer_gradients: Vec<Vec<f64>> = layer_activations
                .iter()
                .map(|x| vec![0.0; x.len()])
                .collect();
            //output layer gralayer_outputs,ds (dE/dA^L)
            layer_gradients[layers - 1] = layer_activations[layers - 1]
                .iter()
                .zip(case.output.iter())
                .map(|(y, t)| d_error_fn(*y, *t))
                .collect();
            // println!("layer results: {:?}", layer_results);
            // println!("last layer grads: {:?}", layer_result_gradients);
            //figure out A^l gradients (dE/dA^l)
            //move from back to front
            for layer_index in (0..(layers - 1)).rev() {
                for i in 0..layer_gradients[layer_index].len() {
                    let mut temp_grad = 0.0;
                    for j in 0..layer_gradients[layer_index + 1].len() {
                        temp_grad += d_activation_fn(layer_outputs[layer_index + 1][j])
                            * self.layers[layer_index + 1].weights[j][i]
                            * layer_gradients[layer_index + 1][j]
                    }
                    layer_gradients[layer_index][i] = temp_grad;
                }
            }
            //this was consolidation step, but now it will be application
            for l in 0..layers {
                let prev_layer_activations = if l == 0 {
                    &input
                } else {
                    &layer_activations[l - 1]
                };
                for j in 0..layer_activations[l].len() {
                    self.layers[l].bias[j] -=
                        rate * d_activation_fn(layer_outputs[l][j]) * layer_gradients[l][j];
                    for i in 0..prev_layer_activations.len() {
                        self.layers[l].weights[j][i] -= rate
                            * d_activation_fn(layer_outputs[l][j])
                            * layer_gradients[l][j]
                            * prev_layer_activations[i];
                    }
                }
            }
            // println!(
            //     "input: {:?}, expected output:{:?}, got:{:?}",
            //     input,
            //     case.output,
            //     layer_results[layers - 1]
            // );
            // println!("layer_results: {:?}", layer_results);
        }
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Pre  Weights: {:?}", x.weights));
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Post Weights: {:?}", x.weights));
        // self.layers
        //     .iter()
        //     .for_each(|x| println!("Post Biases: {:?}", x.bias));
        self.test_all(test_cases, error_fn)
    }
}
