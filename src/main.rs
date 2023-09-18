use std::ops::Range;

use network::Network;

mod layer;
mod network;
mod node;
mod pt1;
mod pt1_5;
mod pt2;

fn main() {
    pt1::runner();
    pt1_5::runner();
    pt2::runner();
}

pub struct GenericTestCase {
    pub input: Vec<f64>,
    pub output: Vec<f64>,
    pub display: String,
}

pub fn run(
    title: &str,
    test_cases: &Vec<GenericTestCase>,
    layers: Range<usize>,
    nodes: Range<usize>,
) {
    let inputs = test_cases[0].input.len();
    let outputs = test_cases[0].output.len();
    println!("=== {} ===", title);
    for layer in layers {
        for node in nodes.clone() {
            run_network(
                Network::new(inputs, outputs, node, layer, None),
                &test_cases,
            );
        }
        println!("------");
    }
}

pub fn run_network(network: Network, test_cases: &Vec<GenericTestCase>) {
    print!(
        "internal layers:\t{},\tinternal nodes:\t{}",
        network.internel_layers(),
        network.internal_nodes(),
    );
    let rounds = 10;
    let mut total_iterations = 0;
    for _ in 0..rounds {
        let mut network = network.clone();
        let learn_errors: Vec<f64> = match network.learn(&test_cases, Some(100_000), None) {
            Ok(l) => l,
            Err(e) => panic!("{}", e),
        };
        match verify(network, test_cases) {
            Ok(_) => (),
            Err(e) => {
                println!("\t\t{}", e);
                return;
            }
        }
        total_iterations += learn_errors.len();
    }
    println!("\t\tOk! (avg iterations: {})", total_iterations / rounds);
}
pub fn verify(mut network: Network, test_cases: &Vec<GenericTestCase>) -> Result<(), String> {
    let error = match network.test_all(test_cases) {
        Ok(r) => r,
        Err(e) => return Err(format!("{}: {}", "auto_learn", e)),
    };
    if error != 0.0 {
        return Err(format!("error: {}", error));
    }
    Ok(())
}
