use crate::traits::generic_game_case::*;
use crate::traits::network_traits::BaseNetwork;
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;

pub fn play_game<
    GameState: Clone + Debug + ToString,
    GameCase: GenericGameCase<GameState>,
    N: Clone,
    Network: BaseNetwork<N>,
>(
    game: GameCase,
    network: &mut Network,
) {
    println!("Training network for {:?}", game.title());
    sleep(Duration::from_secs(5));
    let learning_result = network.learn_game(&game, Some(64), Some(12), Some(10_000));
    match learning_result {
        Ok(r) => println!("Training complete, last error {:.4}", r.last().unwrap()),
        Err(e) => panic!("Training failed: {:?}", e),
    }
    println!("play_game {:?}", game.title());
    let mut game_state: GameState = game.get_empty_initial();
    loop {
        println!("game_state\n{}", game_state.to_string());
        let transformed_state = network.run_game_step(&game, &mut game_state);
        game_state = match &transformed_state {
            StateTransform::Ok(s) => s.clone(),
            StateTransform::Err(invalid_move) => {
                println!("Found invalid move {:?}", invalid_move);
                if !invalid_move.can_continue {
                    break;
                }
                invalid_move.state.clone()
            }
        };
        let game_result = match game.output_result(&game_state, &transformed_state) {
            Ok(result) => result,
            Err(e) => {
                println!("Error {:?}", e);
                break;
            }
        };
        if game_result.game_over {
            println!("You lost?");
            break;
        }

        println!("game_state\n{}", game_state.to_string());
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let input = line.trim().parse::<usize>().unwrap();
        let mut input_vec = vec![0.0; game.output_nodes()];
        input_vec[input] = 1.0;
        game_state = match game.output_state_transformer(&game_state, &input_vec) {
            StateTransform::Ok(s) => s,
            StateTransform::Err(invalid_move) => {
                println!("You submitted an invalid move {:?}", invalid_move);
                if !invalid_move.can_continue {
                    break;
                }
                invalid_move.state
            }
        };
        if game_result.game_over {
            println!("You won?");
            break;
        }
    }
    println!("Game over");
}
