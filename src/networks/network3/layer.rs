use rand::random;

#[derive(Clone)]
pub struct Layer {
    //can we make compile time sized array?
    // pub input_size: usize,
    // pub output_size: usize,
    // f64x4
    pub weights: Vec<Vec<f64>>,
    old_weights: Vec<Vec<f64>>,
    // pub nodes: [[f64;input_size];output_size]],
    pub bias: Vec<f64>,
    old_bias: Vec<f64>,
    output_fn: fn(f64) -> f64,
}
impl Layer {
    pub fn new(input_size: usize, output_size: usize, output_fn: fn(f64) -> f64) -> Layer {
        let rand_rate = || (random::<f64>());
        let weights = (0..output_size)
            .map(|_| (0..input_size).map(|_| rand_rate()).collect())
            .collect();
        let bias = (0..output_size).map(|_| rand_rate()).collect();
        // let weights = (0..output_size)
        //     .map(|_| (0..input_size).map(|_| 0.5).collect())
        //     .collect();
        // let bias = (0..output_size).map(|_| 0.1).collect();

        Layer {
            // weights: vec![vec![0.1; input_size]; output_size],
            weights,
            old_weights: vec![vec![0.0; input_size]; output_size],
            // bias: vec![0.1; output_size],
            bias,
            old_bias: vec![0.0; output_size],
            output_fn,
        }
    }
    pub fn rand_weights(&mut self, rate: f64) {
        let rand_rate = || (random::<f64>() - 0.5) * rate;
        self.old_weights = self.weights.clone();
        self.weights
            .iter_mut()
            .for_each(|x| x.iter_mut().for_each(|y| *y += rand_rate()));
        self.old_bias = self.bias.clone();
        self.bias.iter_mut().for_each(|x| *x += 0.1 * rand_rate());
    }
    pub fn run_total(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.weights
            .iter()
            .map(|paths| {
                paths
                    .iter()
                    .zip(inputs.iter())
                    .map(|(x, y)| x * y)
                    .sum::<f64>()
            })
            .zip(self.bias.iter())
            .map(|(x, y)| (x + y))
            .collect()
    }
    pub fn run(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.run_activate(self.run_total(inputs))
    }
    pub fn run_activate(&self, inputs: Vec<f64>) -> Vec<f64> {
        inputs.iter().map(|x| (self.output_fn)(*x)).collect()
    }

    pub fn revert(&mut self) {
        self.weights = self.old_weights.clone();
        self.bias = self.old_bias.clone();
    }
}
