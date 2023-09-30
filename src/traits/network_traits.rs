use std::{fmt::Debug, usize::MAX};

use rand::random;

use super::{generic_game_case::*, generic_test_case::GenericTestCase};

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
    fn learn_game<I: Copy + Debug + ToString>(
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
            None => 32,
        };
        let rounds = match rounds {
            Some(x) => x,
            None => 8,
        };
        if rounds % 2 != 0 {
            return Err(format!(
                "rounds must be even, to ensure both networks get to start first (rounds: {})",
                rounds,
            ));
        }
        if mutants % 2 != 0 {
            return Err(format!(
                "mutants must be even, to ensure learning rate can better figure out to go up or down (mutants: {})",
                mutants,
            ));
        }
        let max_iterations = match max_iterations {
            Some(x) => x,
            None => 100_000,
        };
        let iterations_for_rate_increase: usize = 500;
        let mut last_rate_change = 0;
        let perfect_games_error = game.expected_error() * rounds as f64;
        // let mut last_down_rate_error = 0.0;
        // let mut last_up_rate_error = 0.0;
        for iteration in 0..max_iterations {
            let mut current_error = 0.0;
            let mut low_change_mutants = vec![self.clone(); mutants / 2];
            let mut high_change_mutants = vec![self.clone(); mutants / 2];
            let mut low_change_mutants_error = vec![0.0; mutants / 2];
            let mut high_change_mutants_error = vec![0.0; mutants / 2];
            low_change_mutants
                .iter_mut()
                .for_each(|x| x.rand_weights(rate * 0.9));
            high_change_mutants
                .iter_mut()
                .for_each(|x| x.rand_weights(rate * 1.1));

            let mut worst_current_error = 0.0;
            for i in 0..(mutants / 2) {
                let mut mutant_error = 0.0;
                current_error = 0.0;
                for j in 0..rounds {
                    match self.run_game(&mut low_change_mutants[i], game, None, j % 2 == 0, 10) {
                        Ok(game_result) => {
                            if !game_result.game_over {
                                println!("ERROR GAME SHOULD BE OVER");
                            }
                            current_error += game_result.error.unwrap_or(1.0);
                            mutant_error += game_result.opponent_error.unwrap_or(1.0);
                        }
                        Err(err) => return Err(format!("{}: {}", "learn_game", err)),
                    };
                }
                low_change_mutants_error[i] = mutant_error;
                mutant_error = 0.0;
                if current_error > worst_current_error {
                    worst_current_error = current_error;
                }
                for j in 0..rounds {
                    match self.run_game(&mut high_change_mutants[i], game, None, j % 2 == 0, 10) {
                        Ok(game_result) => {
                            if !game_result.game_over {
                                println!("ERROR GAME SHOULD BE OVER");
                            }
                            current_error += game_result.error.unwrap_or(1.0);
                            mutant_error += game_result.opponent_error.unwrap_or(1.0);
                        }
                        Err(err) => return Err(format!("{}: {}", "learn_game", err)),
                    };
                }
                high_change_mutants_error[i] = mutant_error;
                if current_error > worst_current_error {
                    worst_current_error = current_error;
                }
                let min_low =
                    low_change_mutants_error
                        .iter()
                        .enumerate()
                        .fold(
                            (0, 0.0),
                            |max, (i, &v)| if v > max.1 { (i, v) } else { max },
                        );
                let min_high =
                    high_change_mutants_error
                        .iter()
                        .enumerate()
                        .fold(
                            (0, 0.0),
                            |max, (i, &v)| if v > max.1 { (i, v) } else { max },
                        );
                if worst_current_error > min_low.1 || worst_current_error > min_high.1 {
                    if min_low < min_high {
                        self.replace_self(&mut low_change_mutants[min_low.0]);
                        last_rate_change = iteration;
                        rate *= 0.99;
                        println!(
                            "found better mutant at low: {}<{}, new rate: {}",
                            min_low.1, worst_current_error, rate
                        );
                    } else {
                        self.replace_self(&mut high_change_mutants[min_high.0]);
                        last_rate_change = iteration;
                        rate *= 1.05;
                        println!(
                            "found better mutant at high: {}<{}, new rate: {}",
                            min_high.1, worst_current_error, rate
                        );
                    }
                }
            }
            if iteration - last_rate_change > iterations_for_rate_increase {
                println!("=====heating up, rate increasing to {:.3}", rate);
                last_rate_change = iteration;
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
        game.output_state_transformer(&current_state, &network_output)
    }

    fn run_game<I: Copy + Debug + ToString>(
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
        // println!("{}: \n{}", i, current_state.to_string());
        enum NextStateResult<I> {
            Result(Result<GameResult, String>),
            NextState(I),
        }
        let mut handle_next_state = |first_round: bool, current_state: I| -> NextStateResult<I> {
            let network: &mut Self;
            network = if first_round { network_a } else { network_b };
            let transformed_state: StateTransform<I> = network.run_game_step(game, &current_state);
            let game_result: GameResult =
                match game.output_result(&current_state, &transformed_state) {
                    Ok(x) => x,
                    Err(invalid_move) => {
                        println!("Finishing game on error: {}", invalid_move);
                        return NextStateResult::Result(Ok(GameResult::new(true, Some(1.0), None)
                            .swap_errors(first_round ^ self_start)));
                    }
                };
            // match &transformed_state {
            //     StateTransform::Ok(s) => println!("{}: OK:\n{}", 0, s.to_string()),
            //     StateTransform::Err(e) => println!("{}: ERR: \n{}", 0, e.state.to_string()),
            // }
            // println!("[{}] {:?}", "run_game", game_result);
            if game_result.game_over {
                // println!("{}", current_state.to_string());
                return NextStateResult::Result(Ok(game_result.swap_errors(!self_start)));
            }
            //this should/will never be hit with output_result generally handling invalid moves
            match transformed_state {
                StateTransform::Ok(state) => NextStateResult::NextState(state),
                StateTransform::Err(invalid_move) => NextStateResult::Result(Err(format!(
                    "run_game: Invalid move(should have been caught): {}",
                    invalid_move.reason
                ))),
            }
        };
        while i < timeout_rounds {
            // println!("First round");
            current_state = match handle_next_state(true, current_state) {
                NextStateResult::Result(game_result) => return game_result,
                NextStateResult::NextState(next_state) => next_state,
            };
            // println!("Response round");
            current_state = match handle_next_state(false, current_state) {
                NextStateResult::Result(game_result) => return game_result,
                NextStateResult::NextState(next_state) => next_state,
            };
            i += 1;
        }
        Err(format!(
            "{}: {} rounds reached, game not over",
            "run_game", timeout_rounds
        ))
    }
}