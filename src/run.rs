use std::{f64::MAX, ops::Range};

use crate::{
    networks::{
        activation_functions::ActivationFunction, network1::network::Network1,
        network2::network::Network2, network3::network::Network3, Networks,
    },
    traits::{generic_test_case::GenericTestCase, network_traits::BaseNetwork},
};

pub fn run<I, O>(
    title: &str,
    network: Networks,
    test_cases: &Vec<GenericTestCase<I, O>>,
    layers: Range<usize>,
    nodes: Range<usize>,
    activation_fns: Option<ActivationFunction>,
    output_activation_fns: Option<ActivationFunction>,
) {
    let inputs = test_cases[0].get_input().len();
    let outputs = test_cases[0].output_nodes;
    println!(
        "=== {} === {} ===, {} test cases",
        title,
        network,
        test_cases.len()
    );
    for layer in layers {
        for node in nodes.clone() {
            match network {
                Networks::Network1 => {
                    let af = match activation_fns {
                        Some(af) => af,
                        None => ActivationFunction::Relu,
                    };
                    let oaf = match output_activation_fns {
                        Some(oaf) => oaf,
                        None => ActivationFunction::Relu,
                    };
                    run_network(
                        Network1::new(inputs, outputs, node, layer, af, oaf),
                        &test_cases,
                        None,
                        None,
                    )
                }
                Networks::Network2 => {
                    let af = match activation_fns {
                        Some(af) => af,
                        None => ActivationFunction::Relu,
                    };
                    let oaf = match output_activation_fns {
                        Some(oaf) => oaf,
                        None => ActivationFunction::Relu,
                    };
                    run_network(
                        Network2::new(inputs, outputs, node, layer, af, oaf),
                        &test_cases,
                        None,
                        None,
                    )
                }
                Networks::Network3 => {
                    let af = match activation_fns {
                        Some(af) => af,
                        None => Network3::activation_fn(),
                    };
                    let oaf = match output_activation_fns {
                        Some(oaf) => oaf,
                        None => Network3::activation_fn(),
                    };
                    run_network(
                        Network3::new(inputs, outputs, node, layer, af, oaf),
                        &test_cases,
                        None,
                        None,
                    )
                }
            }
        }
    }
}

pub fn run_network<I, O>(
    network: impl BaseNetwork,
    test_cases: &Vec<GenericTestCase<I, O>>,
    error_fn: Option<fn(f64, f64) -> f64>,
    d_error_fn: Option<fn(f64, f64) -> f64>,
) {
    print!(
        "internal layers:\t{},\tinternal nodes:\t{}\t",
        network.internel_layers(),
        network.internal_nodes(),
    );
    // thread::sleep(Duration::from_secs(3));
    let mut network = network.clone();
    let learn_errors: Vec<f64> =
        match network.learn(&test_cases, Some(100_000), None, error_fn, d_error_fn) {
            Ok(l) => l,
            // Err(e) => panic!("{}", e),
            Err(e) => {
                println!("{:?}", e);
                vec![MAX; 1]
            }
        };
    match verify(network, test_cases) {
        Ok(_) => {
            print!("Ok!",);
        }
        Err(_) => {
            print!("failed",);
        }
    }
    println!(
        "\titerations: {}\terror:{}",
        learn_errors.len(),
        learn_errors.last().unwrap()
    );
}
pub fn verify<I, O>(
    mut network: impl BaseNetwork,
    test_cases: &Vec<GenericTestCase<I, O>>,
) -> Result<(), String> {
    //ok i don't really care about this output anymore
    let result = match network.test_all(test_cases, None) {
        Ok(r) => r,
        Err(e) => return Err(format!("{}: {}", "auto_learn", e)),
    };
    if result.error > 0.001 {
        return Err(format!("error: {}", result.error));
    }
    Ok(())
}
