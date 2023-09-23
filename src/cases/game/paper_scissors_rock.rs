use rand::random;

use crate::{generic_test_case::GenericGameCase, run_game::run_game};

pub fn runner() {
    run_game(
        "Game, PaperScissorsRock",
        &PaperScissorsRockGame::new_empty(),
        3..5,
        3..7,
    );
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
    // pub fn get_all() -> Vec<PaperScissorsRockGame> {
    //     vec![
    //         PaperScissorsRockGame::new(0),
    //         PaperScissorsRockGame::new(1),
    //         PaperScissorsRockGame::new(2),
    //     ]
    // }
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
        next_state: &Result<PaperScissorsRockGame, String>,
    ) -> Result<(bool, f64, f64), String> {
        let next_state = match next_state {
            Ok(x) => x,
            Err(e) => return Err(e.clone()),
        };
        if next_state.game_complete {
            let result_move = match next_state.input.iter().find(|x| **x == 1) {
                Some(x) => *x,
                None => return Err("Invalid result move".to_string()),
            };
            let initial_input = match initial_state.input.iter().find(|x| **x == 1) {
                Some(x) => *x,
                None => return Err("Invalid initial move".to_string()),
            };
            if result_move == initial_input {
                return Ok((true, 0.5, 0.5));
            }
            if result_move == (initial_input + 1 % 3) {
                return Ok((true, 0.0, 1.0));
            }
            return Ok((true, 1.0, 0.5));
        }
        return Ok((false, 0.5, 0.5));
    }

    fn output_state_transformer(
        &self,
        input: &PaperScissorsRockGame,
        network_output: &Vec<f64>,
    ) -> Result<PaperScissorsRockGame, String> {
        let output: Vec<usize> = network_output
            .iter()
            .map(|x| x.round() as usize)
            .enumerate()
            .filter(|(_, x)| *x == 1)
            .map(|(i, _)| i)
            .collect();
        if output.len() != 3 || output.iter().sum::<usize>() != 1 {
            return Err("Invalid move".to_string());
        }
        let output = [output[0], output[1], output[2]];
        let total: usize = input.input.iter().sum();
        Ok(PaperScissorsRockGame {
            input: output,
            game_complete: total == 1,
        })
    }

    fn input_nodes(&self) -> usize {
        3
    }
}
