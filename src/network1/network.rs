use crate::network_traits::BaseNetwork;
use anyhow::Result;

use super::layer::Layer;

//these really don't need to be structs but they probably will need to be later?
#[derive(Clone)]
pub struct Network1 {
    pub layers: Vec<Layer>,
    pub output_fn: fn(f64) -> f64,
}

impl BaseNetwork<Network1> for Network1 {
    fn new(
        input_nodes: usize,
        output_nodes: usize,
        internal_nodes: usize,
        internal_layers: usize,
        output_fn: Option<fn(f64) -> f64>,
    ) -> Network1 {
        Network1 {
            layers: {
                let mut layers: Vec<Layer> = Vec::new();
                layers.push(Layer::new(internal_nodes, input_nodes));
                for _ in 0..(internal_layers - 1) {
                    layers.push(Layer::new(internal_nodes, internal_nodes));
                }
                layers.push(Layer::new(output_nodes, internal_nodes));
                layers
            },
            output_fn: match output_fn {
                Some(x) => x,
                None => |x| x.min(1.0).max(0.0),
                // None => |x| x,
            },
        }
    }
    fn replace_self(&mut self, other: &mut Self) {
        self.layers = other.layers.clone();
    }
    fn internel_layers(&self) -> usize {
        self.layers.len() - 1
    }
    fn internal_nodes(&self) -> usize {
        if self.layers.len() < 2 {
            return 0;
        }
        self.layers[0].nodes.len()
    }
    fn rand_weights(&mut self, rate: f64) {
        self.layers.iter_mut().for_each(|x| x.rand_weights(rate));
    }
    fn run(&mut self, initial_inputs: &Vec<f64>) -> Result<Vec<f64>, String> {
        if self.layers.len() == 0 {
            return Err("Network: Can not run network with zero layers".to_string());
        }
        if self.layers[0].nodes.len() == 0 {
            return Err("Network: Can not run network with zero nodes".to_string());
        }
        if self.layers[0].nodes[0].paths.len() == 0 {
            return Err("Network: Can not run network with zero paths".to_string());
        }
        if initial_inputs.len() != self.layers[0].nodes[0].paths.len() {
            return Err(format!(
                "{}: initial_inputs {} != layers.first.len {})",
                "Network::run",
                initial_inputs.len(),
                self.layers[0].nodes.len()
            ));
        }
        self.layers[0].run(&initial_inputs)?;
        for i in 1..self.layers.len() {
            let inputs = &self.layers[i - 1]
                .nodes
                .iter()
                .map(|x| x.value)
                .collect::<Vec<f64>>();
            self.layers[i].run(inputs)?;
        }
        let output_fn = self.output_fn;
        match self.layers.last() {
            Some(x) => Ok(x
                .nodes
                .iter()
                .map(|x| output_fn(x.value))
                .collect::<Vec<f64>>()),
            None => Err("self.layers.last() returned None".to_string()),
        }
    }

    fn revert(&mut self) {
        self.layers.iter_mut().for_each(|x| x.revert());
    }
}
