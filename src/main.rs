use clap::{Parser, Subcommand};
use generic_test_case::GenericTestCase;

mod cases;
mod generic_test_case;
mod network1;
mod network_traits;
mod run;
mod run_game;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    runner: Option<Runner>,
}

#[derive(Subcommand)]
enum Runner {
    Game,
    Simple,
}

fn main() {
    let cli = Cli::parse();
    match cli.runner {
        Some(Runner::Game) => cases::game::runner(),
        Some(Runner::Simple) => cases::simple::runner(),
        None => {
            cases::simple::runner();
            cases::game::runner();
        }
    }
}
