use rand::random;

#[derive(Clone)]
pub struct Node {
    //initially i thought paths would be paths out, but i think paths in would actually be better
    //(so that's what I'm trying first)
    pub paths: Vec<f64>,
    pub old_paths: Vec<f64>,
    pub value: f64,
}

impl Node {
    pub fn new(len: usize) -> Node {
        Node {
            paths: vec![0.5; len],
            old_paths: vec![0.5; len],
            value: 0.5,
        }
    }
    pub fn run(&mut self, inputs: &Vec<f64>) -> Result<(), String> {
        if inputs.len() != self.paths.len() {
            return Err(format!(
                "inputs.len() != self.paths.len() ({} != {})",
                inputs.len(),
                self.paths.len()
            ));
        }
        self.value = 0.0;
        for i in 0..inputs.len() {
            self.value += inputs[i] * self.paths[i];
        }
        self.value = self.value.min(1.0);
        Ok(())
    }
    pub fn rand_weights(&mut self, rate: f64) {
        // for i in 0..self.paths.len() {
        //     self.paths[i] += (random::<f64>() - 0.5) * rate;
        // }
        self.paths
            .iter_mut()
            .for_each(|x| *x += (random::<f64>() - 0.5) * rate);
    }

    pub fn result(&mut self, learn: bool) {
        if learn {
            //if result is worse than previous step
            self.paths = self.old_paths.clone();
        }
    }
}
