use crate::{layer::Layer, GenericTestCase};
use anyhow::Result;

//these really don't need to be structs but they probably will need to be later?
pub struct Network {
    pub layers: Vec<Layer>,
}
impl Network {
    pub fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
    ) -> Network {
        Network {
            layers: {
                let mut layers: Vec<Layer> = Vec::new();
                layers.push(Layer::new(internal_nodes, input_nodes));
                for _ in 0..(internal_layers - 1) {
                    layers.push(Layer::new(internal_nodes, internal_nodes));
                }
                layers.push(Layer::new(output_nodes, internal_nodes));
                layers
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
        match self.layers.last() {
            Some(x) => Ok(x.nodes.iter().map(|x| x.value).collect::<Vec<f64>>()),
            None => Err("self.layers.last() returned None".to_string()),
        }
    }

    //result is the value compared to previous success rate, 1.0 would be same as previous
    // result is a ratio (higher is better)
    pub fn result(&mut self, learn: bool) {
        if learn {
            self.layers.iter_mut().for_each(|x| x.result(learn));
        }
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
        println!(
            "result: {:?}\nexpect: {:?}, result_diff: {:?}",
            result, test_case.output, result_difference
        );
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

    pub fn auto_learn(
        &mut self,
        test_cases: &Vec<GenericTestCase>,
        max_iterations: usize,
    ) -> Result<(), String> {
        let mut learn_results = vec![0.0; max_iterations];
        let mut prev_result = self.test_all(&test_cases)?;
        let mut rate: f64 = 0.1;
        for i in 0..max_iterations {
            let result = match self.test_all(&test_cases) {
                Ok(r) => r,
                Err(e) => return Err(format!("{}: {}", "auto_learn", e)),
            };
            let learn = result < prev_result;
            self.result(learn);
            learn_results[i] = result;
            if learn {
                rate *= 0.9;
            } else {
                rate *= 1.05;
            }
            rate = rate.min(1.0).max(0.0);
            println!("i:{} result:{} rate:{}", i, result, rate);
            println!("{}", self.display());
            self.rand_weights(rate);
            prev_result = result;
        }
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
