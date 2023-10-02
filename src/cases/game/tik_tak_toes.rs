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
    //
    //actually test would be better but just vague test, ensuring differences where there should be
}
#[cfg(test)]
mod tests {
    use crate::cases::game::tik_tak_toes::{BoardSpace, TikTakToes, TikTakToesState};
    use crate::traits::generic_game_case::{GenericGameCase, InvalidMove, StateTransform};

    fn handle_transform(x: StateTransform<TikTakToesState>) -> TikTakToesState {
        match x {
            StateTransform::Ok(state) => state,
            StateTransform::Err(invalid_move) => {
                assert!(false, "Invalid move: {:?}", invalid_move);
                invalid_move.state
            }
        }
    }
    fn transform(state: &TikTakToesState, network_output: &Vec<f64>) -> TikTakToesState {
        let transformed_state = TikTakToes.output_state_transformer(&state, &network_output);
        handle_transform(transformed_state)
    }
    fn assert_board(board: &TikTakToesState, x: &Vec<(usize, usize)>, o: &Vec<(usize, usize)>) {
        let format_reason: String = format!(
            "Got:\n{}\nExpectedX:{:?}\nExpectedO:{:?}",
            board.to_string(),
            x,
            o
        );
        for i in 0..3 {
            for j in 0..3 {
                if x.contains(&(i, j)) {
                    assert_eq!(board.board[i][j], BoardSpace::X, "{}", format_reason);
                } else if o.contains(&(i, j)) {
                    assert_eq!(board.board[i][j], BoardSpace::O, "{}", format_reason);
                } else {
                    assert_eq!(board.board[i][j], BoardSpace::Empty, "{}", format_reason);
                }
            }
        }
    }
    fn turn(
        state: &TikTakToesState,
        x_c: usize,
        y_c: usize,
        x: &mut Vec<(usize, usize)>,
        o: &mut Vec<(usize, usize)>,
    ) -> TikTakToesState {
        assert!(x_c < 3 && y_c < 3);
        match state.next_turn() {
            BoardSpace::Empty => (),
            BoardSpace::X => x.push((x_c, y_c)),
            BoardSpace::O => o.push((x_c, y_c)),
        }
        let index = x_c * 3 + y_c;
        let mut network_output = vec![0.0; 9];
        network_output[index] = 1.0;
        let temp = transform(&state, &network_output);
        assert_board(&temp, &x, &o);
        temp
    }

    #[test]
    fn correct_move() {
        let mut state = TikTakToesState::new();
        let mut x_coords: Vec<(usize, usize)> = vec![];
        let mut o_coords: Vec<(usize, usize)> = vec![];

        //round 1 x win
        let turns = vec![(0, 0), (1, 0), (1, 1), (1, 2)];
        turns
            .iter()
            .for_each(|(x, y)| state = turn(&state, *x, *y, &mut x_coords, &mut o_coords));
        assert!(state.valid());
        assert_eq!(state.winner(), BoardSpace::Empty);
        state = turn(&state, 2, 2, &mut x_coords, &mut o_coords);
        assert_eq!(
            state.winner(),
            BoardSpace::X,
            "State:\n{}",
            state.to_string()
        );
        //round 2 o win
        x_coords.clear();
        o_coords.clear();
        let turns = vec![(0, 0), (2, 0), (1, 1), (2, 2), (1, 2)];
        state = TikTakToesState::new();
        turns
            .iter()
            .for_each(|(x, y)| state = turn(&state, *x, *y, &mut x_coords, &mut o_coords));
        assert!(state.valid());
        assert_eq!(state.winner(), BoardSpace::Empty);
        assert!(!state.game_over());
        state = turn(&state, 2, 1, &mut x_coords, &mut o_coords);
        assert_eq!(state.winner(), BoardSpace::O);
        assert!(state.game_over());
        //round 3
        x_coords.clear();
        o_coords.clear();
        let turns = vec![
            //XOX
            //OOX
            //XXO
            (0, 0),
            (1, 1),
            (0, 2),
            (0, 1),
            (2, 1),
            (1, 0),
            (2, 0),
            (2, 2),
            (1, 2),
        ];
        state = TikTakToesState::new();
        turns
            .iter()
            .for_each(|(x, y)| state = turn(&state, *x, *y, &mut x_coords, &mut o_coords));
        assert!(state.valid());
        assert_eq!(state.winner(), BoardSpace::Empty);
        assert!(state.game_over());
    }
    fn transformed_error(
        state: &TikTakToesState,
        network_output: &Vec<f64>,
    ) -> InvalidMove<TikTakToesState> {
        let transformed_state = TikTakToes.output_state_transformer(&state, &network_output);
        assert!(transformed_state.is_err());
        transformed_state.unwrap_err()
    }
    fn transformed_error_new(network_output: &Vec<f64>) -> InvalidMove<TikTakToesState> {
        transformed_error(&TikTakToesState::new(), network_output)
    }
    #[test]
    fn multiple_same_max_values() {
        let mut network_output = vec![0.0; 9];
        network_output[0] = 1.0;
        network_output[1] = 1.0;
        let err_2_inputs = transformed_error_new(&network_output);
        assert!(!err_2_inputs.can_continue);
    }
    fn invalid_move_comparison(network_output: &Vec<f64>, network_output_2: &Vec<f64>) {
        let err1 = transformed_error_new(&network_output);
        let err2 = transformed_error_new(&network_output_2);
        assert!(!err1.can_continue);
        assert!(!err2.can_continue);
        assert!(err1.error < err2.error, "Expected first network to have lower error than second\nErr1:{}\nErr2:{}\nInput1:{:?}\nInput2:{:?}",
        err1.error,
        err2.error,
        network_output,
        network_output_2
    );
    }
    fn move_comparison(
        state: &TikTakToesState,
        network_output: &Vec<f64>,
        network_output_2: &Vec<f64>,
        can_be_equal: bool,
        game_over: Option<bool>,
    ) {
        let transformed_state = TikTakToes.output_state_transformer(&state, &network_output);
        let transformed_state_2 = TikTakToes.output_state_transformer(&state, &network_output);
        let result = match TikTakToes.output_result(&state, &transformed_state) {
            Ok(r) => r,
            Err(e) => panic!("Error: {}", e),
        };
        let result_2 = match TikTakToes.output_result(&state, &transformed_state_2) {
            Ok(r) => r,
            Err(e) => panic!("Error: {}", e),
        };
        match game_over {
            Some(g) => {
                assert_eq!(
                    g, result.game_over,
                    "Result1 game over: {}, expected: {}",
                    result.game_over, g
                );
                assert_eq!(
                    g, result_2.game_over,
                    "Result2 game over: {}, expected: {}",
                    result.game_over, g
                );
            }
            None => (),
        }
        let comparison_result = if can_be_equal {
            result.error <= result_2.error
        } else {
            result.error < result_2.error
        };
        assert!(comparison_result, "Expected first network to have lower error than second\nErr1:{:?}\nErr2:{:?}\nInput1:{:?}\nInput2:{:?}",
        result.error,
        result_2.error,
        network_output,
        network_output_2
    );
    }
    #[test]
    fn invalid_move() {
        let mut network_output = vec![0.0; 9];
        network_output[1] = 0.4;
        let mut network_output_2 = vec![0.0; 9];
        network_output_2[1] = 0.3;
        invalid_move_comparison(&network_output, &network_output_2);
        let mut network_output = vec![0.1; 9];
        network_output[1] = 0.4;
        let mut network_output_2 = vec![0.2; 9];
        network_output_2[1] = 0.45;
        invalid_move_comparison(&network_output, &network_output_2);
        let mut network_output = vec![0.0; 9];
        network_output[1] = 1.0;
        network_output[4] = 1.0;
        let mut network_output_2 = vec![0.1; 9];
        network_output_2[1] = 0.4;
        invalid_move_comparison(&network_output, &network_output_2);
    }
    #[test]
    fn output_move_comparison() {
        let mut state = TikTakToesState::new();
        let mut network_output = vec![0.0; 9];
        network_output[1] = 1.0;
        let mut network_output_2 = vec![0.0; 9];
        network_output_2[0] = 1.0;
        move_comparison(
            &state,
            &network_output,
            &network_output_2,
            true,
            Some(false),
        );
        state.board[0][0] = BoardSpace::X;
        move_comparison(
            &state,
            &network_output,
            &network_output_2,
            false,
            Some(false),
        );
    }
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
        // Ok(GameResult::new(false, Some(0.5), Some(0.5)))
        Ok(GameResult::new(false, Some(0.0), Some(0.0)))
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
