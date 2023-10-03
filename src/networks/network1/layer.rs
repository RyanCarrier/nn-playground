use super::node::Node;

#[derive(Clone)]
pub struct Layer {
    pub nodes: Vec<Node>,
}
impl Layer {
    pub fn new(len: usize, inputs: usize) -> Layer {
        Layer {
            nodes: vec![Node::new_default(inputs); len],
        }
    }
    pub fn rand_weights(&mut self, rate: f64) {
        self.nodes.iter_mut().for_each(|x| x.rand_weights(rate));
    }
    pub fn run(&mut self, inputs: &Vec<f64>) -> Result<(), String> {
        if self.nodes.len() == 0 {
            return Err("self.nodes.len() == 0".to_string());
        }
        self.nodes.iter_mut().for_each(|x| x.run(&inputs).unwrap());
        Ok(())
    }
    pub fn revert(&mut self) {
        self.nodes.iter_mut().for_each(|x| x.revert());
    }
}
