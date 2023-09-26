use clap::{Args, Parser, Subcommand};

mod cases;
mod network1;
mod run;
mod run_game;
mod traits;

#[derive(Parser, Debug)]
#[command(name = "nn")]
struct Cli {
    #[command(subcommand)]
    runner: Option<Runner>,
}

#[derive(Debug, Subcommand)]
enum Runner {
    Game,
    Data(DataArgs),
}

#[derive(Debug, Args)]
struct DataArgs {
    #[command(subcommand)]
    data: Option<DataSet>,
}

#[derive(Debug, Subcommand)]
enum DataSet {
    Or,
    And,
    AndOr,
}

fn main() {
    let cli = Cli::parse();
    match cli.runner {
        Some(Runner::Game) => cases::game::runner(),
        Some(Runner::Data(d)) => match d.data {
            Some(DataSet::Or) => cases::simple::or::runner(),
            Some(DataSet::And) => cases::simple::and::runner(),
            //lol andor orand
            Some(DataSet::AndOr) => cases::simple::or_and::runner(),
            None => cases::simple::runner(),
        },
        None => {
            cases::simple::runner();
            cases::game::runner();
        }
    }
}
