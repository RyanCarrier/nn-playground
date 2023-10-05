//this needs to just be turned into traits lol
pub struct GenericTestCase<I, O> {
    pub input: I,
    pub input_transformer: fn(&I) -> Vec<f64>,
    pub output: Vec<f64>,
    pub output_nodes: usize,
    pub output_transformer: fn(Vec<f64>) -> O,
    pub display: String,
    //output_error returns error from 0.0 (correct) to 1.0 (inverse)
    // pub output_error: fn(&Vec<f64>, &Vec<f64>) -> f64,
}

impl<I, O> GenericTestCase<I, O> {
    pub fn get_input(&self) -> Vec<f64> {
        (self.input_transformer)(&self.input)
    }
    pub fn output_error(&self, result: Vec<f64>, expected: Vec<f64>) -> f64 {
        result
            .iter()
            .zip(expected.iter())
            .map(|x| (x.0 - x.1).powi(2))
            .sum::<f64>()
            / expected.len() as f64
    }
}

pub struct BatchResult {
    pub results: Vec<TestResult>,
    pub error: f64,
}
impl PartialEq for BatchResult {
    fn eq(&self, other: &Self) -> bool {
        self.error == other.error
    }
}
impl PartialOrd for BatchResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.error.partial_cmp(&other.error)
    }
}
impl BatchResult {
    pub fn new(results: Vec<TestResult>) -> BatchResult {
        let error = results.iter().map(|x| x.error).sum::<f64>() / results.len() as f64;
        BatchResult { results, error }
    }
}

pub struct TestResult {
    pub result: Vec<f64>,
    pub expected: Vec<f64>,
    pub error: f64,
}
