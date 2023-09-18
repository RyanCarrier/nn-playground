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
            paths: vec![0.0; len],
            old_paths: vec![0.0; len],
            value: 0.0,
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
        fn rand_rate(rate: f64) -> f64 {
            (random::<f64>() - 0.5) * rate
        }
        self.paths.iter_mut().for_each(|x| {
            let r = rand_rate(rate);
            // println!("r: {}, rate: {}", r, rate);
            *x += r;
            // *x = x.min(1.0).max(-1.0);
            *x = x.min(1.0).max(0.0);
        });
        // println!("old: {:?}, new:{:?}", old, self.paths);
    }

    pub fn result(&mut self, learn: bool) {
        if learn {
            //if result is worse than previous step
            self.paths = self.old_paths.clone();
        }
    }
}
