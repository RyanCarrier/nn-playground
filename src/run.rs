use std::ops::Range;

use crate::{generic_test_case::GenericTestCase, network1::network::Network1};

pub fn run<I, O>(
    title: &str,
    test_cases: &Vec<GenericTestCase<I, O>>,
    layers: Range<usize>,
    nodes: Range<usize>,
) {
    let inputs = test_cases[0].get_input().len();
    let outputs = test_cases[0].output_nodes;
    println!("=== {} ===", title);
    for layer in layers {
        for node in nodes.clone() {
            run_network(
                Network1::new(inputs, outputs, node, layer, None),
                &test_cases,
            );
        }
        println!("------");
    }
}

pub fn run_network<I, O>(network: Network1, test_cases: &Vec<GenericTestCase<I, O>>) {
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
pub fn verify<I, O>(
    mut network: Network1,
    test_cases: &Vec<GenericTestCase<I, O>>,
) -> Result<(), String> {
    let error = match network.test_all(test_cases) {
        Ok(r) => r,
        Err(e) => return Err(format!("{}: {}", "auto_learn", e)),
    };
    if error != 0.0 {
        return Err(format!("error: {}", error));
    }
    Ok(())
}
