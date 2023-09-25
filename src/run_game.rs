use std::{fmt::Debug, ops::Range};

use crate::{
    generic_test_case::GenericGameCase, network1::network::Network1, network_traits::BaseNetwork,
};

pub fn run_game<I: Copy + Debug>(
    title: &str,
    game: &impl GenericGameCase<I>,
    layers: Range<usize>,
    nodes: Range<usize>,
) {
    println!("=== {} ===", title);
    let outputs = game.output_nodes();
    let inputs = game.input_nodes();
    for layer in layers {
        for node in nodes.clone() {
            run_game_network(
                Network1::new(inputs, outputs, node, layer, Some(|x| x)),
                game,
            );
        }
        println!("------");
    }
}

pub fn run_game_network<I: Copy + Debug>(network: Network1, game: &impl GenericGameCase<I>) {
    print!(
        "internal layers:\t{},\tinternal nodes:\t{}",
        network.internel_layers(),
        network.internal_nodes(),
    );
    let mut latest_error: f64 = 1.0;
    let rounds = 10;
    let mut total_iterations = 0;
    for _ in 0..rounds {
        let mut network = network.clone();
        let learn_errors = match network.learn_game(game, None, None, None) {
            Ok(l) => l,
            Err(e) => {
                println!("run_game_network learn_game error: {}", e);
                return;
            }
        };
        total_iterations = learn_errors.len();
        latest_error = *learn_errors.last().unwrap();
    }
    println!(
        "\t\tOk! (avg iterations: {}) last_error: {:.3}\t\tI only say ok because there is no not okay for games yet",
        total_iterations / rounds,
        latest_error
    );
}
