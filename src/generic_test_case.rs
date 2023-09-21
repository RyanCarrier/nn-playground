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
    ) -> Result<I, String>;

    fn get_random_initial(&self) -> I;
    fn get_empty_initial(&self) -> I;
    /// Returns (game_over, error, opponent_error)
    fn output_result(
        &self,
        initial_state: &I,
        next_state: &Result<I, String>,
    ) -> Result<(bool, f64, f64), String>;
}
