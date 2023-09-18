use rand::random;

#[derive(Clone)]
pub struct Node {
    //initially i thought paths would be paths out, but i think paths in would actually be better
    //(so that's what I'm trying first)
    pub paths: Vec<f64>,
    pub old_paths: Vec<f64>,
    pub c: f64,
    pub old_c: f64,
    pub value: f64,
}

impl Node {
    pub fn new(default_weight: f64, len: usize) -> Node {
        Node {
            paths: vec![default_weight; len],
            old_paths: vec![default_weight; len],
            c: 0.0,
            old_c: 0.0,
            value: 0.0,
        }
    }
    pub fn new_default(len: usize) -> Node {
        Node::new(0.0, len)
    }
    pub fn new_path_c(paths: Vec<f64>, c: f64) -> Node {
        Node {
            old_paths: paths.clone(),
            paths,
            c,
            old_c: c,
            value: 0.0,
        }
    }
    pub fn new_paths(paths: Vec<f64>) -> Node {
        Node {
            old_paths: paths.clone(),
            paths,
            c: 0.0,
            old_c: 0.0,
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
        self.value = self.c;
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
            if random::<f64>() > 0.5 {
                //only update half of them
                return;
            }
            let r = rand_rate(rate);
            // println!("r: {}, rate: {}", r, rate);
            *x += r;
            *x = x.min(1.0).max(-1.0);
            // *x = x.min(1.0).max(0.0);
        });
        if random::<f64>() > 0.5 {
            //only update half of them
            self.c = (self.c + rand_rate(rate)).min(1.0).max(-1.0);
        }
        // println!("old: {:?}, new:{:?}", old, self.paths);
    }

    pub fn revert(&mut self) {
        //if result is worse than previous step
        self.paths = self.old_paths.clone();
        self.c = self.old_c.clone();
    }
    pub fn update(&mut self) {
        //lock in
        self.old_paths = self.paths.clone();
        self.old_c = self.c.clone();
    }
}
