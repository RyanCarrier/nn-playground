//this needs to just be turned into traits lol
pub struct GenericTestCase<I, O> {
    pub input: I,
    pub input_transformer: fn(&I) -> Vec<f64>,
    pub output: O,
    pub output_nodes: usize,
    pub output_transformer: fn(Vec<f64>) -> O,
    pub display: String,
    //output_error returns error from 0.0 (correct) to 1.0 (inverse)
    pub output_error: fn(&O, &O) -> f64,
}

impl<I, O> GenericTestCase<I, O> {
    pub fn get_input(&self) -> Vec<f64> {
        (self.input_transformer)(&self.input)
    }
    pub fn result_error(&self, result: O) -> f64 {
        (self.output_error)(&self.output, &result)
    }
}

pub trait GenericGameCase<I: Copy> {
    fn input_transformer(&self, input: &I) -> Vec<f64>;
    fn input_nodes(&self) -> usize;
    fn output_nodes(&self) -> usize;
    fn output_state_transformer(
        &self,
        initial_state: &I,
        network_output: &Vec<f64>,
    ) -> StateTransform<I>;

    fn get_random_initial(&self) -> I;
    fn get_empty_initial(&self) -> I;
    /// Returns (game_over, error, opponent_error)
    fn output_result(
        &self,
        initial_state: &I,
        next_state: &StateTransform<I>,
    ) -> Result<GameResult, String>;
    fn invalid_move_error(&self, initial_state: &I, network_output: &Vec<f64>) -> f64;
}
#[derive(Debug)]
pub enum StateTransform<State> {
    Ok(State),
    Err(InvalidMove<State>),
}

#[derive(Debug)]
pub struct InvalidMove<State> {
    pub state: State,
    pub error: f64,
    pub reason: String,
    pub can_continue: bool,
}
impl<T> Into<GameResult> for InvalidMove<T> {
    fn into(self) -> GameResult {
        GameResult {
            game_over: !self.can_continue,
            error: Some(self.error),
            //if there is an invalid move, we don't know how 'wrong' the opponent is
            opponent_error: None,
        }
    }
}
impl<T> Into<GameResult> for &InvalidMove<T> {
    fn into(self) -> GameResult {
        GameResult {
            game_over: !self.can_continue,
            error: Some(self.error),
            //if there is an invalid move, we don't know how 'wrong' the opponent is
            opponent_error: None,
        }
    }
}
#[derive(Debug)]
pub struct GameResult {
    pub game_over: bool,
    pub error: Option<f64>,
    pub opponent_error: Option<f64>,
}
impl GameResult {
    pub fn new(game_over: bool, error: Option<f64>, opponent_error: Option<f64>) -> GameResult {
        GameResult {
            game_over,
            error,
            opponent_error,
        }
    }
    pub fn win() -> GameResult {
        GameResult {
            game_over: true,
            error: Some(0.0),
            opponent_error: Some(1.0),
        }
    }
    pub fn loss() -> GameResult {
        Self::win().swap_errors(true)
    }
    pub fn tie() -> GameResult {
        GameResult {
            game_over: true,
            error: Some(0.5),
            opponent_error: Some(0.5),
        }
    }
    pub fn swap_errors(mut self, swap: bool) -> Self {
        if swap {
            let temp = self.error;
            self.error = self.opponent_error;
            self.opponent_error = temp;
        }
        self
    }
}
