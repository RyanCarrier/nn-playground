use std::{fmt::Debug, usize::MAX};

use crate::generic_test_case::{
    GameResult, GenericGameCase, GenericTestCase, InvalidMove, StateTransform,
};

pub trait BaseNetwork<T: Clone>: Clone {
    fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        output_fn: Option<fn(f64) -> f64>,
    ) -> T;
    fn internel_layers(&self) -> usize;
    fn internal_nodes(&self) -> usize;
    fn rand_weights(&mut self, rate: f64);
    fn run(&mut self, initial_inputs: &Vec<f64>) -> Result<Vec<f64>, String>;

    //result is the value compared to previous success rate, 1.0 would be same as previous
    // result is a ratio (higher is better)
    fn revert(&mut self);
    fn replace_self(&mut self, other: &mut Self);

    fn test<I, O>(&mut self, test_case: &GenericTestCase<I, O>) -> Result<f64, String> {
        let result = match self.run(&test_case.get_input()) {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "test", err)),
        };
        let result_difference: f64 = test_case.result_error((test_case.output_transformer)(result));
        Ok(result_difference)
    }

    //returns the average difference between the output and the expected output (0.0 is perfect, 1.0
    //is opposite)
    fn test_all<I, O>(&mut self, test_cases: &Vec<GenericTestCase<I, O>>) -> Result<f64, String> {
        let cases_len = test_cases.len();
        let results: Vec<f64> = match test_cases.into_iter().map(|x| self.test(x)).collect() {
            Ok(x) => x,
            Err(err) => return Err(format!("{}: {}", "test_all", err)),
        };
        let result = results.into_iter().sum::<f64>() / cases_len as f64;
        Ok(result)
    }
    fn print_all<I, O>(&mut self, test_cases: &Vec<GenericTestCase<I, O>>) -> Result<(), String> {
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
    fn auto_learn<I, O>(
        &mut self,
        test_cases: &Vec<GenericTestCase<I, O>>,
    ) -> Result<Vec<f64>, String> {
        //we probably should have a timeout heh
        self.learn(test_cases, None, Some(0.00000001))
    }
    fn learn<I, O>(
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

    ///--------- Game Specific Functions ---------

    /// Return whether game is over, and the current error (0.0 is win, 1.0 is loss),
    /// in refernce to both players (error, oponent error)
    /// 0.5 if inconclusive or tie ie; current player makes illegal move, so (true, 1.0, 0.5)
    /// while a tie would be (true, 0.5, 0.5)
    ///
    /// Also this should be self not mut, we shouldn't store values in nodes, that's how we would
    /// turn this more matricie like too
    fn test_round<I: Copy>(
        &mut self,
        game: &impl GenericGameCase<I>,
    ) -> Result<GameResult, String> {
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
    fn learn_game<I: Copy + Debug>(
        &mut self,
        game: &impl GenericGameCase<I>,
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
            None => 6,
        };
        let rounds = match rounds {
            Some(x) => x,
            None => 10,
        };
        let max_iterations = match max_iterations {
            Some(x) => x,
            None => 100_000,
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
                        Ok(game_result) => {
                            if !game_result.game_over {
                                println!("ERROR GAME SHOULD BE OVER");
                            }
                            current_error += game_result.error.unwrap_or(1.0);
                            mutant_error += game_result.opponent_error.unwrap_or(1.0);
                            // println!(
                            //     "=====learn_game, round: {}, current: {:.2}, mutant: {:.2}",
                            //     i, current_error, mutant_error
                            // );
                        }
                        Err(err) => return Err(format!("{}: {}", "learn_game", err)),
                    };
                }
                //it's probably better to just waste storage and complexity to RR all the mutants?
                // or atleast compare with self for the BEST one
                if i % 10 == 0 {
                    println!(
                        "learn_game, current: {:.2}, mutant: {:.2}, rate: {:.3}",
                        current_error, mutant_error, rate
                    );
                }
                if mutant_error < current_error {
                    println!(
                        "=====learn_game, current: {:.2}, mutant: {:.2}, rate: {:.3}",
                        current_error, mutant_error, rate
                    );
                    self.replace_self(&mut mutant_network);
                    last_rate_change = i;
                    rate *= 0.99;
                }
            }
            if i - last_rate_change > 100 {
                println!("=====heating up, rate increasing to {:.3}", rate);
                last_rate_change = i;
                rate *= 1.05;
            }
            errors.push(current_error);
        }
        Ok(errors)
    }

    fn run_game_step<I: Copy + Debug>(
        &mut self,
        game: &impl GenericGameCase<I>,
        current_state: &I,
    ) -> StateTransform<I> {
        let network_input = game.input_transformer(&current_state);
        let network_output = match self.run(&network_input) {
            Ok(x) => x,
            Err(err) => {
                return StateTransform::Err(InvalidMove {
                    state: current_state.clone(),
                    error: game.invalid_move_error(&current_state, &network_input),
                    reason: format!("{}: {}", "run_game_step", err),
                    can_continue: false,
                })
            }
        };
        // println!("{:?} -> {:?}", current_state, network_output);
        let next_state = game.output_state_transformer(&current_state, &network_output);
        next_state
    }

    fn run_game<I: Copy + Debug>(
        &mut self,
        opponent_network: &mut Self,
        game: &impl GenericGameCase<I>,
        initial_state: Option<&I>,
        self_start: bool,
        timeout_rounds: usize,
    ) -> Result<GameResult, String> {
        let initial_state = match initial_state {
            Some(x) => *x,
            None => game.get_empty_initial(),
        };
        let mut current_state = initial_state.clone();
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
            let next_state: StateTransform<I> = network_a.run_game_step(game, &current_state);
            // println!(
            //     "Starting round {}: {:?}, next_state:{:?}",
            //     i, current_state, next_state
            // );
            let game_result: GameResult = match game.output_result(&current_state, &next_state) {
                Ok(x) => x,
                Err(_) => {
                    return Ok(GameResult::new(true, Some(1.0), None).swap_errors(!self_start))
                }
            };
            // println!("{}: {:?} -> {:?}", i, current_state, game_result);
            if game_result.game_over {
                return Ok(game_result.swap_errors(!self_start));
            }
            current_state = match next_state {
                StateTransform::Ok(state) => state,
                StateTransform::Err(invalid_move) => {
                    return Err(format!(
                        "run_game: Invalid move(should have been caught): {}",
                        invalid_move.reason
                    ));
                }
            };
            let next_state = network_b.run_game_step(game, &current_state);
            let game_result = match game.output_result(&current_state, &next_state) {
                Ok(x) => x,
                Err(_) => return Ok(GameResult::new(true, Some(1.0), None).swap_errors(self_start)),
            };
            // println!("{}: {:?} -> {:?}", i, current_state, game_result);
            if game_result.game_over {
                return Ok(game_result.swap_errors(self_start));
            }
            current_state = match next_state {
                StateTransform::Ok(state) => state,
                StateTransform::Err(invalid_move) => {
                    panic!(
                        "Invalid move: {}\n this should have already been handled",
                        invalid_move.reason
                    );
                }
            };
            i += 1;
        }
        Err(format!(
            "{}: {} rounds reached, game not over",
            "run_game", timeout_rounds
        ))
    }
}
