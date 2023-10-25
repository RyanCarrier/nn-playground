use std::ops::Range;

use crate::{
    networks::{
        network1::network::Network1, network2::network::Network2, network3::network::Network3,
        Networks,
    },
    traits::{generic_test_case::GenericTestCase, network_traits::BaseNetwork},
};

pub fn run<I, O>(
    title: &str,
    network: Networks,
    test_cases: &Vec<GenericTestCase<I, O>>,
    layers: Range<usize>,
    nodes: Range<usize>,
) {
    let inputs = test_cases[0].get_input().len();
    let outputs = test_cases[0].output_nodes;
    println!("=== {} === {} ===", title, network);
    for layer in layers {
        for node in nodes.clone() {
            let af = |x: f64| x.max(0.0);
            let daf = |x: f64| if x > 0.0 { 1.0 } else { 0.0 };
            match network {
                Networks::Network1 => run_network(
                    Network1::new(inputs, outputs, node, layer, af),
                    &test_cases,
                    None,
                    None,
                    daf,
                ),
                Networks::Network2 => run_network(
                    Network2::new(inputs, outputs, node, layer, af),
                    &test_cases,
                    None,
                    None,
                    daf,
                ),
                Networks::Network3 => run_network(
                    Network3::new(inputs, outputs, node, layer, af),
                    &test_cases,
                    None,
                    None,
                    daf,
                ),
            }
        }

        println!("------");
    }
}

pub fn run_network<I, O>(
    network: impl BaseNetwork,
    test_cases: &Vec<GenericTestCase<I, O>>,
    error_fn: Option<fn(f64, f64) -> f64>,
    d_error_fn: Option<fn(f64, f64) -> f64>,
    d_activation_fn: fn(f64) -> f64,
) {
    print!(
        "internal layers:\t{},\tinternal nodes:\t{}",
        network.internel_layers(),
        network.internal_nodes(),
    );
    let rounds = 10;
    let mut total_iterations = 0;
    for _ in 0..rounds {
        let mut network = network.clone();
        let learn_errors: Vec<f64> = match network.learn(
            &test_cases,
            Some(100_000),
            None,
            error_fn,
            d_error_fn,
            d_activation_fn,
        ) {
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
    mut network: impl BaseNetwork,
    test_cases: &Vec<GenericTestCase<I, O>>,
) -> Result<(), String> {
    let result = match network.test_all(test_cases, None) {
        Ok(r) => r,
        Err(e) => return Err(format!("{}: {}", "auto_learn", e)),
    };
    if result.error != 0.0 {
        return Err(format!("error: {}", result.error));
    }
    Ok(())
}
