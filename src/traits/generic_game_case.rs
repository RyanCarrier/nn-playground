pub trait GenericGameCase<I: Clone> {
    fn input_transformer(&self, input: &I) -> Vec<f64>;
    fn title(&self) -> &str;
    fn input_nodes(&self) -> usize;
    fn expected_error(&self) -> f64;
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
#[allow(dead_code)]
impl<State> StateTransform<State> {
    pub const fn is_ok(&self) -> bool {
        match self {
            StateTransform::Ok(_) => true,
            StateTransform::Err(_) => false,
        }
    }
    pub const fn is_err(&self) -> bool {
        match self {
            StateTransform::Ok(_) => false,
            StateTransform::Err(_) => true,
        }
    }
    pub fn unwrap(self) -> State {
        match self {
            StateTransform::Ok(s) => s,
            StateTransform::Err(e) => e.state,
        }
    }
    pub fn unwrap_err(self) -> InvalidMove<State> {
        match self {
            StateTransform::Ok(_) => panic!("Called unwrap_err on Ok"),
            StateTransform::Err(e) => e,
        }
    }
}

#[derive(Debug)]
pub struct InvalidMove<State> {
    pub state: State,
    pub error: f64,
    pub reason: String,
    pub can_continue: bool,
    pub network_output: Vec<f64>,
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
