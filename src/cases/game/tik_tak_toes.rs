use crate::run_game::run_game;
use crate::traits::generic_game_case::*;

pub fn runner() {
    run_game("Game, TikTakToes", &TikTakToes {}, 10..11, 10..11);
}
pub fn game_test() {
    //this is where I'm going to test various specific network outputs and seeing what we end
    //up at.
    //I'm not sure if we need exact tests for this as we are still activaly changin it
    //I more just want to see what's going on and figure out why it's not working
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum BoardSpace {
    Empty,
    X,
    O,
}
#[derive(Clone, Copy, Debug)]
pub struct TikTakToesState {
    pub board: [[BoardSpace; 3]; 3],
}
impl TikTakToesState {
    pub fn new() -> Self {
        Self {
            board: [[BoardSpace::Empty; 3]; 3],
        }
    }
    #[allow(dead_code)]
    pub fn valid(&self) -> bool {
        let mut x: usize = 0;
        let mut o: usize = 0;
        self.board.iter().flatten().for_each(|a| match a {
            BoardSpace::X => x += 1,
            BoardSpace::O => o += 1,
            BoardSpace::Empty => (),
        });
        x >= o
    }
    pub fn next_turn(&self) -> BoardSpace {
        let mut x: usize = 0;
        let mut o: usize = 0;
        let mut e: usize = 0;
        self.board.iter().flatten().for_each(|a| match a {
            BoardSpace::X => x += 1,
            BoardSpace::O => o += 1,
            BoardSpace::Empty => e += 1,
        });
        if x > o {
            BoardSpace::O
        } else {
            BoardSpace::X
        }
    }
    pub fn game_over(&self) -> bool {
        self.board
            .iter()
            .flatten()
            .filter(|x| **x != BoardSpace::Empty)
            .count()
            >= 9
            || self.winner() != BoardSpace::Empty
    }
    pub fn winner(&self) -> BoardSpace {
        for i in 0..3 {
            if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] {
                return self.board[i][0].clone();
            }
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
                return self.board[0][i].clone();
            }
        }
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            return self.board[0][0].clone();
        }
        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            return self.board[0][2].clone();
        }
        return BoardSpace::Empty;
    }
}
impl ToString for TikTakToesState {
    fn to_string(&self) -> String {
        self.board
            .map(|x| {
                x.map(|y| match y {
                    BoardSpace::Empty => '_',
                    BoardSpace::X => 'X',
                    BoardSpace::O => 'O',
                })
                .iter()
                .collect::<String>()
            })
            .join("\n")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TikTakToes;

impl GenericGameCase<TikTakToesState> for TikTakToes {
    fn title(&self) -> &str {
        "TikTakToes"
    }
    fn get_random_initial(&self) -> TikTakToesState {
        todo!();
    }
    fn get_empty_initial(&self) -> TikTakToesState {
        TikTakToesState::new()
    }

    fn input_transformer(&self, input: &TikTakToesState) -> Vec<f64> {
        input
            .board
            .iter()
            .flatten()
            .map(|x| (*x as u8) as f64)
            .collect()
    }
    fn input_nodes(&self) -> usize {
        9
    }
    fn output_nodes(&self) -> usize {
        9
    }
    fn expected_error(&self) -> f64 {
        0.5
    }

    fn output_result(
        &self,
        _initial_state: &TikTakToesState,
        next_state: &StateTransform<TikTakToesState>,
    ) -> Result<GameResult, String> {
        let next_state = match next_state {
            StateTransform::Ok(state) => state,
            StateTransform::Err(invalid_move) => {
                // println!("Found invalid move {:?}", invalid_move);
                if !invalid_move.can_continue {
                    return Ok(invalid_move.into());
                }
                &invalid_move.state
            }
        };
        if next_state.game_over() {
            return Ok(match next_state.winner() {
                BoardSpace::Empty => GameResult::tie(),
                BoardSpace::X => GameResult::win(),
                BoardSpace::O => GameResult::loss(),
            });
        }
        Ok(GameResult::new(false, Some(0.5), Some(0.5)))
    }
    fn invalid_move_error(&self, input: &TikTakToesState, network_output: &Vec<f64>) -> f64 {
        let mut error = 0.0;
        let (max_i, max) = network_output
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        match *max {
            //this case should never happen
            x if x < 0.0 => error += (x - 1.0).abs() * 0.1,
            //this is more likely
            x if x < 0.5 => error += (x - 1.0).abs() * 0.05,
            x if x < 1.0 => error += (x - 1.0).abs() * 0.005,
            x => error += (x - 1.0).abs() * 0.1,
        }
        network_output
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != max_i)
            .for_each(|(_, x)| {
                error += x.abs() / 5.0;
                if x == max {
                    error += 5.0;
                }
            });
        if input.board[max_i / 3][max_i % 3] != BoardSpace::Empty {
            error += 1.0;
        }
        // print!("Invalid error max: {:.2}\t", max);
        error
    }

    fn output_state_transformer(
        &self,
        input: &TikTakToesState,
        network_output: &Vec<f64>,
    ) -> StateTransform<TikTakToesState> {
        // let next_move: Vec<usize> = network_output
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, x)| **x >= 0.5)
        //     .map(|(i, _)| i)
        //     .collect();
        // if next_move.len() != 1 {
        //     return StateTransform::Err(InvalidMove {
        //         state: input.clone(),
        //         error: self.invalid_move_error(input, network_output),
        //         reason: "Output should be a single index".to_string(),
        //         can_continue: false,
        //     });
        // }
        // let next_move = next_move[0];
        // print!("[");
        // network_output.iter().for_each(|x| print!("{:.4}, ", x));
        // println!("]");
        let next_move: (usize, f64) = network_output.iter().enumerate().fold(
            (0, 0.0),
            |max, (i, &v)| if v > max.1 { (i, v) } else { max },
        );
        if next_move.1 < 0.5 {
            return StateTransform::Err(InvalidMove {
                state: input.clone(),
                error: self.invalid_move_error(input, network_output),
                reason: format!(
                    "Output should be atleast higher weight than 0.5 (was {})",
                    next_move.1
                ),
                can_continue: false,
                network_output: network_output.clone(),
            });
        }
        let next_move = next_move.0;

        let next_move_piece = input.next_turn();
        if input.board[next_move / 3][next_move % 3] != BoardSpace::Empty {
            return StateTransform::Err(InvalidMove {
                state: input.clone(),
                error: self.invalid_move_error(input, network_output),
                reason: format!("Output board move space is not empty ({})", next_move),
                can_continue: false,
                network_output: network_output.clone(),
            });
        }
        let mut output_state = input.clone();
        output_state.board[next_move / 3][next_move % 3] = next_move_piece;
        StateTransform::Ok(output_state)
    }
}
