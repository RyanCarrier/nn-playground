use crate::run_game::run_game;
use crate::traits::generic_game_case::*;

pub fn runner() {
    run_game("Game, TikTakToes", &TikTakToes {}, 3..5, 5..7);
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
    pub fn to_string(&self) -> String {
        self.board
            .map(|x| {
                x.map(|y| match y {
                    BoardSpace::Empty => ' ',
                    BoardSpace::X => 'X',
                    BoardSpace::O => 'O',
                })
                .iter()
                .collect::<String>()
            })
            .join("\n")
    }
    pub fn valid(&self) -> bool {
        let mut x: usize = 0;
        let mut o: usize = 0;
        let mut e: usize = 0;
        self.board.iter().flatten().for_each(|a| match a {
            BoardSpace::X => x += 1,
            BoardSpace::O => o += 1,
            BoardSpace::Empty => e += 1,
        });
        x >= o

    }
    pub fn next_turn(&self) -> BoardSpace{
        let mut x: usize = 0;
        let mut o: usize = 0;
        let mut e: usize = 0;
        self.board.iter().flatten().for_each(|a| match a {
            BoardSpace::X => x += 1,
            BoardSpace::O => o += 1,
            BoardSpace::Empty => e += 1,
        });

        if e>=9 {
            return BoardSpace::Empty;
        }
        if x > o{
            return BoardSpace::O;
        }
        return BoardSpace::X;
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

#[derive(Clone, Copy, Debug)]
pub struct TikTakToes;

impl GenericGameCase<TikTakToesState> for TikTakToes {
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
        initial_state: &TikTakToesState,
        next_state: &StateTransform<TikTakToesState>,
    ) -> Result<GameResult, String> {
        let next_state = match next_state {
            StateTransform::Ok(state) => state,
            StateTransform::Err(invalid_move) => {
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
            x if x < 0.5 => error += (x - 1.0).abs() * 5.0,
            x if x < 1.0 => error += (x - 1.0).abs() * 0.5,
            x => error += (x - 1.0).abs() * 0.1,
        }
        network_output
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != max_i)
            .for_each(|(_, x)| error += x.abs() / 5.0);
        // println!("invalid move error: {}", error);
        error
    }

    fn output_state_transformer(
        &self,
        input: &TikTakToesState,
        network_output: &Vec<f64>,
    ) -> StateTransform<TikTakToesState> {
        let output: Vec<usize> = network_output
            .iter()
            .enumerate()
            .filter(|(_, x)| **x >= 0.5)
            .map(|(i, _)| i)
            .collect();
        // println!("output: {:?}", output);
        if output.len() != 1 {
            return StateTransform::Err(InvalidMove {
                state: input.clone(),
                error: self.invalid_move_error(input, network_output),
                reason: "Output should be a single index".to_string(),
                can_continue: false,
            });
        }
let next_move_piece = input.next_turn();
        if input.board[output[0]/3][output[0]%3] != BoardSpace::Empty {
            return StateTransform::Err(InvalidMove {
                state: input.clone(),
                error: self.invalid_move_error(input, network_output),
                reason: "Output should be a single index".to_string(),
                can_continue: false,
            });
        }
        let mut output_state = input.clone();
        output_state.board[output[0]/3][output[0]%3] = next_move_piece;
        
        if output[0] > 2 {
            return StateTransform::Err(InvalidMove {
                state: input.clone(),
                error: self.invalid_move_error(input, network_output),
                reason: "Output should be a single index".to_string(),
                can_continue: false,
            });
        }
        input.board[output[0]] = BoardSpace::X;
        let mut next_input: [usize; 3] = [0; 3];
        next_input[output[0]] = 1;
        StateTransform::Ok(TikTakToesState { board:  })
    }
}
