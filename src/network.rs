use std::usize::{self, MAX};

use crate::{layer::Layer, GenericTestCase};
use anyhow::Result;

//these really don't need to be structs but they probably will need to be later?
pub struct Network {
    pub layers: Vec<Layer>,
    pub output_fn: fn(f64) -> f64,
}
impl Network {
    pub fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        output_fn: Option<fn(f64) -> f64>,
    ) -> Network {
        Network {
            layers: {
                let mut layers: Vec<Layer> = Vec::new();
                layers.push(Layer::new(internal_nodes, input_nodes));
                for _ in 0..(internal_layers) {
                    layers.push(Layer::new(internal_nodes, internal_nodes));
                }
                layers.push(Layer::new(output_nodes, internal_nodes));
                layers
            },
            output_fn: match output_fn {
                Some(x) => x,
                None => |x| x.min(1.0).max(0.0),
            },
        }
    }
    pub fn rand_weights(&mut self, rate: f64) {
        self.layers.iter_mut().for_each(|x| x.rand_weights(rate));
    }
    pub fn run(&mut self, initial_inputs: &Vec<f64>) -> Result<Vec<f64>, String> {
        if self.layers.len() == 0 {
            return Err("Network: Can not run network with zero layers".to_string());
        }
        if self.layers[0].nodes.len() == 0 {
            return Err("Network: Can not run network with zero nodes".to_string());
        }
        if self.layers[0].nodes[0].paths.len() == 0 {
            return Err("Network: Can not run network with zero paths".to_string());
        }
        if initial_inputs.len() != self.layers[0].nodes[0].paths.len() {
            return Err(format!(
                "{}: initial_inputs {} != layers.first.len {})",
                "Network::run",
                initial_inputs.len(),
                self.layers[0].nodes.len()
            ));
        }
        for i in 0..self.layers.len() {
            if i == 0 {
                self.layers[i].run(&initial_inputs)?;
                continue;
            }
            let inputs = &self.layers[i - 1]
                .nodes
                .iter()
                .map(|x| x.value)
                .collect::<Vec<f64>>();
            self.layers[i].run(inputs)?;
        }
        let output_fn = self.output_fn;
        match self.layers.last() {
            Some(x) => Ok(x
                .nodes
                .iter()
                .map(|x| output_fn(x.value))
                .collect::<Vec<f64>>()),
            None => Err("self.layers.last() returned None".to_string()),
        }
    }

    //result is the value compared to previous success rate, 1.0 would be same as previous
    // result is a ratio (higher is better)
    pub fn update(&mut self) {
        self.layers.iter_mut().for_each(|x| x.update());
    }
    pub fn revert(&mut self) {
        self.layers.iter_mut().for_each(|x| x.revert());
    }

    //returns the difference between the output and the expected output (0.0 is perfect, 1.0 is
    //opposite)
    pub fn test(&mut self, test_case: &GenericTestCase) -> Result<f64, String> {
        let result = match self.run(&test_case.input) {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "test", err)),
        };
        let result_difference = result
            .iter()
            .zip(test_case.output.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<f64>()
            / (test_case.output.len() as f64);
        // if result_difference > 0.1 {
        //     println!(
        //         "===RESULT DIFFERENCE===, result: {:?} expect: {:?}, result_diff: {:?}",
        //         result, test_case.output, result_difference
        //     );
        // }
        Ok(result_difference)
    }

    //returns the average difference between the output and the expected output (0.0 is perfect, 1.0
    //is opposite)
    pub fn test_all(&mut self, test_cases: &Vec<GenericTestCase>) -> Result<f64, String> {
        let cases_len = test_cases.len();
        let results: Vec<f64> = match test_cases.into_iter().map(|x| self.test(x)).collect() {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "test_all", err)),
        };
        let result = results.into_iter().sum::<f64>() / cases_len as f64;
        Ok(result)
    }

    pub fn print_all(&mut self, test_cases: &Vec<GenericTestCase>) -> Result<(), String> {
        let cases_len = test_cases.len();
        for i in 0..cases_len {
            let result = match self.run(&test_cases[i].input) {
                Ok(x) => x,
                Err(err) => return Err(format!("{}: {}", "print_all", err)),
            };
            println!("===case {}===\n{}", i, &test_cases[i].display);
            println!(
                "test_result: [{}], diff: [{}]",
                result[0],
                (result[0] - &test_cases[i].output[0]).abs()
            );
        }
        Ok(())
    }
    pub fn auto_learn(&mut self, test_cases: &Vec<GenericTestCase>) -> Result<(), String> {
        //we probably should have a timeout heh
        self.learn(test_cases, None, Some(0.00000001))
    }
    pub fn learn(
        &mut self,
        test_cases: &Vec<GenericTestCase>,
        max_iterations: Option<usize>,
        min_error: Option<f64>,
    ) -> Result<(), String> {
        let mut learn_error = Vec::new();
        let mut error = self.test_all(&test_cases)?;
        let mut rate: f64 = 0.2;
        let max_iterations = match max_iterations {
            Some(x) => x,
            None => MAX,
        };
        let min_error = match min_error {
            Some(x) => x,
            None => 0.0,
        };

        let mut i = 0;
        let mut best_error = 1.0;
        let mut last_rate_change = 0;
        while i < max_iterations && error > min_error {
            self.rand_weights(rate);
            error = match self.test_all(&test_cases) {
                Ok(r) => r,
                Err(e) => return Err(format!("{}: {}", "auto_learn", e)),
            };
            if error < best_error {
                best_error = error;
                rate *= 0.99;
                last_rate_change = i;
                println!("=====learn, rate lowering to {:.3}", rate);
                //this is innefficient
                self.update();
            } else {
                self.revert();
            }
            if i - last_rate_change > 50 {
                rate *= 1.05;
                println!("=====heating up, rate increasing to {:.3}", rate);
                last_rate_change = i;
            }
            learn_error.push(error);
            rate = rate.min(4.0).max(0.0);
            println!("{}: {}", i, error);
            i += 1;
        }
        let result = self.test_all(&test_cases)?;
        println!(
            "==RESULT==\ni:{} error:{} rate:{}",
            max_iterations, result, rate
        );
        println!("==VALUES==\n{}", self.display());
        self.print_all(&test_cases)?;
        Ok(())
    }
    pub fn display(&self) -> String {
        let mut result = String::new();
        self.layers.iter().for_each(|x| {
            result.push_str(&x.display());
            result.push_str("\n");
        });
        result
    }
}
