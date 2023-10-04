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
        Self::new_paths(vec![default_weight; len])
    }
    pub fn new_default(len: usize) -> Node {
        Node::new(0.0, len)
        // Node::new_path_c(vec![0.0; len], 0.5)
    }
    pub fn new_path_c(paths: Vec<f64>, c: f64) -> Node {
        Node {
            old_paths: paths.clone(),
            paths,
            c,
            old_c: c.clone(),
            value: 0.0,
        }
    }
    pub fn new_paths(paths: Vec<f64>) -> Node {
        Self::new_path_c(paths, 0.0)
    }
    pub fn run(&mut self, inputs: &Vec<f64>) -> Result<(), String> {
        if inputs.len() != self.paths.len() {
            return Err(format!(
                "inputs.len() != self.paths.len() ({} != {})",
                inputs.len(),
                self.paths.len()
            ));
        }
        self.value = inputs
            .iter()
            .zip(self.paths.iter())
            .map(|(x, y)| x * y)
            .sum::<f64>()
            + self.c;
        //TODO: Test which is faster, garuntee it's the above one, but be cool to see how much
        // then also to check how this compares to doing more matricie style
        // self.value = self.c;
        // for i in 0..inputs.len() {
        //     self.value += inputs[i] * self.paths[i];
        // }
        self.value = self.value.min(1.0).max(0.0);
        //i think my and one is fucked, becuase removing this bounds seems to
        //break the and one, but the AND one seems broken cause the AND+OR one
        //seems to run better... lol
        Ok(())
    }
    pub fn rand_weights(&mut self, rate: f64) {
        //-0.5 to 0.5
        let rand_rate = || (random::<f64>() - 0.5) * rate;
        self.old_paths = self.paths.clone();
        self.old_c = self.c.clone();
        self.paths.iter_mut().for_each(|x| {
            let r = rand_rate();
            // println!("r: {}, rate: {}", r, rate);
            *x += r;
            //removing these path limits seemed to work fine
            // *x = x.min(1.0).max(-1.0);
            // *x = x.min(1.0).max(0.0);
        });
        //only update half of them
        //this one is also fine without bounds
        // self.c = (self.c + rand_rate(rate)).min(1.0).max(-1.0);
        // self.c += 0.1 * rand_rate(rate);
        self.c += 0.1 * rand_rate();
        // println!("old: {:?}, new:{:?}", self.old_paths, self.paths);
        // println!("oldc: {:?}, newc:{:?}", self.old_c, self.c);
    }

    pub fn revert(&mut self) {
        //if result is worse than previous step
        self.paths = self.old_paths.clone();
        self.c = self.old_c.clone();
    }
}
