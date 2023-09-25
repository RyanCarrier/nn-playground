use rand::random;

use crate::{
    generic_test_case::{GameResult, GenericGameCase, InvalidMove, StateTransform},
    run_game::run_game,
};

pub fn runner() {
    run_game(
        "Game, PaperScissorsRock",
        &PaperScissorsRockGame::new_empty(),
        5..7,
        5..7,
    );
}

#[cfg(test)]
mod tests {
    use super::GenericGameCase;
    use super::PaperScissorsRockGame;

    struct TestCaseInvalidMove {
        input: [f64; 3],
        output: f64,
    }

    #[test]
    fn test_paper_scissors_rock() {
        let test_cases = vec![
            TestCaseInvalidMove {
                input: [1.0, 0.0, 0.0],
                output: 0.0,
            },
            TestCaseInvalidMove {
                input: [0.0, 1.0, 0.0],
                output: 0.0,
            },
            TestCaseInvalidMove {
                input: [0.0, 0.0, 1.0],
                output: 0.0,
            },
            // bad inputs
            TestCaseInvalidMove {
                input: [0.0, 0.0, 0.0],
                output: 5.0,
            },
            // worse inputs
            TestCaseInvalidMove {
                input: [1.0, 1.0, 0.0],
                output: 10.0,
            },
            TestCaseInvalidMove {
                input: [1.0, 1.0, 1.0],
                output: 20.0,
            },
            TestCaseInvalidMove {
                input: [10.0, 1.0, 1.0],
                output: 20.0 + (9.0 / 2.0),
            },
        ];
        //max above 1.0 is /2, max under 1.0 is *5
        //every non max is *10
        let game = PaperScissorsRockGame::new_empty();
        for case in test_cases {
            let result = game.invalid_move_error(&game, &case.input.to_vec());
            assert_eq!(result, case.output, "input: {:?}", case.input);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PaperScissorsRockGame {
    pub input: [usize; 3],
    pub game_complete: bool,
}

impl PaperScissorsRockGame {
    pub fn new(i: usize) -> PaperScissorsRockGame {
        let mut input = [0; 3];
        input[i] = 1;
        PaperScissorsRockGame {
            input,
            game_complete: false,
        }
    }
    pub fn new_empty() -> PaperScissorsRockGame {
        let input = [0; 3];
        PaperScissorsRockGame {
            input,
            game_complete: false,
        }
    }
}
impl GenericGameCase<PaperScissorsRockGame> for PaperScissorsRockGame {
    fn get_random_initial(&self) -> PaperScissorsRockGame {
        PaperScissorsRockGame::new(random::<usize>() % 3)
    }
    fn get_empty_initial(&self) -> PaperScissorsRockGame {
        PaperScissorsRockGame::new_empty()
    }

    fn input_transformer(&self, input: &PaperScissorsRockGame) -> Vec<f64> {
        input.input.iter().map(|x| *x as f64).collect()
    }

    fn output_nodes(&self) -> usize {
        3
    }

    fn output_result(
        &self,
        initial_state: &PaperScissorsRockGame,
        next_state: &StateTransform<PaperScissorsRockGame>,
    ) -> Result<GameResult, String> {
        let next_state = match next_state {
            StateTransform::Ok(state) => state,
            StateTransform::Err(invalid_move) => {
                if !invalid_move.can_continue {
                    // println!(
                    //     "output_result: can't continue, Invalid move: {}",
                    //     invalid_move.reason
                    // );
                    return Ok(invalid_move.into());
                }
                &invalid_move.state
            }
        };
        if next_state.game_complete {
            let result_move = match next_state.input.iter().find(|x| **x >= 1) {
                Some(x) => *x,
                None => return Ok(GameResult::new(true, Some(1.0), Some(0.0))),
            };
            let initial_input = match initial_state.input.iter().find(|x| **x == 1) {
                Some(x) => *x,
                None => return Err("Invalid initial move".to_string()),
            };
            if result_move == initial_input {
                return Ok(GameResult::tie());
            }
            if result_move == (initial_input + 1 % 3) {
                return Ok(GameResult::win());
            }
            return Ok(GameResult::loss());
        }
        Ok(GameResult::new(false, Some(0.5), Some(0.5)))
    }
    fn invalid_move_error(&self, input: &PaperScissorsRockGame, network_output: &Vec<f64>) -> f64 {
        let mut error = 0.0;
        let (max_i, max) = network_output
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        if *max < 1.0 {
            error += (max - 1.0).abs() * 5.0
        } else {
            error += (max - 1.0).abs() / 2.0
        }
        network_output.iter().enumerate().for_each(|(i, x)| {
            if i != max_i {
                error += x.abs() * 10.0;
            }
        });
        // println!("invalid move error: {}", error);
        error
    }

    fn output_state_transformer(
        &self,
        input: &PaperScissorsRockGame,
        network_output: &Vec<f64>,
    ) -> StateTransform<PaperScissorsRockGame> {
        let output: Vec<usize> = network_output
            .iter()
            .enumerate()
            .filter(|(_, x)| **x >= 0.5)
            .map(|(i, _)| i)
            .collect();
        // println!("output: {:?}", output);
        if output.len() != 1 || output[0] > 2 {
            return StateTransform::Err(InvalidMove {
                state: input.clone(),
                error: self.invalid_move_error(input, network_output),
                reason: "Output should be a single index".to_string(),
                can_continue: false,
            });
        }
        let mut next_input: [usize; 3] = [0; 3];
        next_input[output[0]] = 1;
        StateTransform::Ok(PaperScissorsRockGame {
            input: next_input,
            game_complete: input.input.iter().sum::<usize>() > 0,
        })
    }

    fn input_nodes(&self) -> usize {
        3
    }
}
