use std::usize::{self, MAX};

use anyhow::Result;

use crate::{generic_test_case::GenericGameCase, GenericTestCase};

use super::layer::Layer;

//these really don't need to be structs but they probably will need to be later?
#[derive(Clone)]
pub struct Network1 {
    pub layers: Vec<Layer>,
    pub output_fn: fn(f64) -> f64,
}
impl Network1 {
    pub fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        output_fn: Option<fn(f64) -> f64>,
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
            output_fn: match output_fn {
                Some(x) => x,
                None => |x| x.min(1.0).max(0.0),
            },
        }
    }
    pub fn internel_layers(&self) -> usize {
        self.layers.len() - 1
    }
    pub fn internal_nodes(&self) -> usize {
        if self.layers.len() < 2 {
            return 0;
        }
        self.layers[0].nodes.len()
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

    /// Return whether game is over, and the current error (0.0 is win, 1.0 is loss),
    /// in refernce to both players (error, oponent error)
    /// 0.5 if inconclusive or tie ie; current player makes illegal move, so (true, 1.0, 0.5)
    /// while a tie would be (true, 0.5, 0.5)
    ///
    /// Also this should be self not mut, we shouldn't store values in nodes, that's how we would
    /// turn this more matricie like too
    pub fn test_round<T: Copy>(
        &mut self,
        game: &impl GenericGameCase<T>,
    ) -> Result<(bool, f64, f64), String> {
        let initial_state = game.get_random_initial();
        let network_input = game.input_transformer(&initial_state);
        let network_output = match self.run(&network_input) {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "test", err)),
        };
        let result = game.output_state_transformer(&initial_state, &network_output);
        game.output_result(&initial_state, &result)
    }

    //returns the historic learn errors
    pub fn learn_game<T: Copy>(
        &mut self,
        game: &impl GenericGameCase<T>,
        //mutants, sets how many mutations to try
        mutants: Option<usize>,
        //how many rounds should we run before choosing a winner mutant
        rounds: Option<usize>,
        //how many iterations of updating the network should we do
        max_iterations: Option<usize>,
    ) -> Result<Vec<f64>, String> {
        let mut rate: f64 = 0.2;
        let mut errors: Vec<f64> = Vec::new();
        let mutants = match mutants {
            Some(x) => x,
            None => 10,
        };
        let rounds = match rounds {
            Some(x) => x,
            None => 100,
        };
        let max_iterations = match max_iterations {
            Some(x) => x,
            None => 10_000,
        };
        let mut last_rate_change = 0;
        for i in 0..max_iterations {
            let mut current_error = 0.0;
            for _ in 0..mutants {
                let mut mutant_network = self.clone();
                mutant_network.rand_weights(rate);
                let mut mutant_error = 0.0;
                current_error = 0.0;
                for i in 0..rounds {
                    match self.run_game(&mut mutant_network, game, None, i % 2 == 0, 4) {
                        Ok((game_over, error, opponent_error)) => {
                            if !game_over {
                                println!("ERROR GAME SHOULD BE OVER");
                            }
                            current_error += error;
                            mutant_error += opponent_error;
                        }
                        Err(err) => return Err(format!("{}: {}", "learn_game", err)),
                    };
                }
                //it's probably better to just waste storage and complexity to RR all the mutants?
                // or atleast compare with self for the BEST one
                if mutant_error < current_error {
                    self.layers = mutant_network.layers.clone();
                    last_rate_change = i;
                    rate *= 0.99;
                }
            }
            if i - last_rate_change > 10 {
                last_rate_change = i;
                rate *= 1.05;
            }
            errors.push(current_error);
        }
        Ok(errors)
    }

    pub fn run_game_step<T: Copy>(
        &mut self,
        game: &impl GenericGameCase<T>,
        current_state: &T,
    ) -> Result<T, String> {
        let network_input = game.input_transformer(&current_state);
        let network_output = match self.run(&network_input) {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "run_game", err)),
        };
        let next_state = game.output_state_transformer(&current_state, &network_output);
        next_state
    }

    pub fn run_game<T: Copy>(
        &mut self,
        opponent_network: &mut Self,
        game: &impl GenericGameCase<T>,
        initial_state: Option<&T>,
        self_start: bool,
        timeout_rounds: usize,
    ) -> Result<(bool, f64, f64), String> {
        let initial_state = match initial_state {
            Some(x) => *x,
            None => game.get_empty_initial(),
        };
        let mut current_state: T = initial_state.clone();
        let network_a: &mut Self;
        let network_b: &mut Self;
        if self_start {
            network_a = self;
            network_b = opponent_network;
        } else {
            network_a = opponent_network;
            network_b = self;
        }
        let mut i = 0;
        while i < timeout_rounds {
            let next_state = network_a.run_game_step(game, &current_state);
            let (game_over, error, opponent_error) =
                match game.output_result(&current_state, &next_state) {
                    Ok(x) => x,
                    // Err(err) => return Err(format!("{}: {}", "run_game", err)),
                    Err(_) => {
                        if self_start {
                            return Ok((true, 1.0, 0.0));
                        }
                        return Ok((true, 0.0, 1.0));
                    }
                };
            if game_over {
                if self_start {
                    return Ok((game_over, error, opponent_error));
                }
                return Ok((game_over, opponent_error, error));
            }
            current_state = next_state.unwrap();
            let next_state = network_b.run_game_step(game, &current_state);
            let (game_over, error, opponent_error) =
                match game.output_result(&current_state, &next_state) {
                    Ok(x) => x,
                    // Err(err) => return Err(format!("{}: {}", "run_game", err)),
                    Err(_) => {
                        if self_start {
                            return Ok((true, 0.0, 1.0));
                        }
                        return Ok((true, 1.0, 0.0));
                    }
                };
            if game_over {
                if self_start {
                    return Ok((game_over, opponent_error, error));
                }
                return Ok((game_over, error, opponent_error));
            }
            current_state = next_state.unwrap();
            i += 1;
        }
        Err(format!(
            "{}: {} rounds reached, game not over",
            "run_game", timeout_rounds
        ))
        // println!(
        //     "game over after ? rounds, error: {}, opponent_error: {}",
        //     error, opponent_error
        // );
    }

    //returns the difference between the output and the expected output (0.0 is perfect, 1.0 is
    //opposite)
    pub fn test<I, O>(&mut self, test_case: &GenericTestCase<I, O>) -> Result<f64, String> {
        let result = match self.run(&test_case.get_input()) {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "test", err)),
        };
        let result_difference: f64 = test_case.result_error((test_case.output_transformer)(result));
        Ok(result_difference)
    }

    //returns the average difference between the output and the expected output (0.0 is perfect, 1.0
    //is opposite)
    pub fn test_all<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
    ) -> Result<f64, String> {
        let cases_len = test_cases.len();
        let results: Vec<f64> = match test_cases.into_iter().map(|x| self.test(x)).collect() {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "test_all", err)),
        };
        let result = results.into_iter().sum::<f64>() / cases_len as f64;
        Ok(result)
    }

    pub fn print_all<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
    ) -> Result<(), String> {
        let cases_len = test_cases.len();
        for i in 0..cases_len {
            let result = match self.run(&test_cases[i].get_input()) {
                Ok(x) => x,
                Err(err) => return Err(format!("{}: {}", "print_all", err)),
            };
            println!("===case {}===\n{}", i, &test_cases[i].display);
            println!(
                "test_result: [{}], diff: [{}]",
                result[0].clone(),
                test_cases[i].result_error((test_cases[i].output_transformer)(result))
            );
        }
        Ok(())
    }
    pub fn auto_learn<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
    ) -> Result<Vec<f64>, String> {
        //we probably should have a timeout heh
        self.learn(test_cases, None, Some(0.00000001))
    }
    pub fn learn<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
        max_iterations: Option<usize>,
        min_error: Option<f64>,
    ) -> Result<Vec<f64>, String> {
        let mut learn_errors = Vec::new();
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
                // println!("=====learn, rate lowering to {:.3}", rate);
                //this is innefficient
                self.update();
            } else {
                self.revert();
            }
            if i - last_rate_change > 50 {
                rate *= 1.05;
                // println!("=====heating up, rate increasing to {:.3}", rate);
                last_rate_change = i;
            }
            learn_errors.push(error);
            rate = rate.min(4.0).max(0.0);
            // println!("{}: {}", i, error);
            i += 1;
        }
        Ok(learn_errors)
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
