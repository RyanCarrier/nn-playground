use cases::game::{play::play_game, tik_tak_toes::TikTakToes};
use clap::{Args, Parser, Subcommand};
use network1::network::Network1;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use traits::network_traits::BaseNetwork;

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
    Game(GameArgs),
    PlayGame(GameArgs),
    Data(DataArgs),
}

#[derive(Debug, Args)]
struct GameArgs {
    #[command(subcommand)]
    data: Option<GameSet>,
}

#[derive(Debug, Subcommand, EnumIter)]
enum GameSet {
    PaperScissorsRock,
    TikTakToes,
}

#[derive(Debug, Args)]
struct DataArgs {
    #[command(subcommand)]
    data: Option<DataSet>,
}

#[derive(Debug, Subcommand, EnumIter)]
enum DataSet {
    Or,
    And,
    AndOr,
}

fn main() {
    let cli = Cli::parse();
    run_with(cli.runner);
}

fn run_with(runner: Option<Runner>) {
    match runner {
        Some(Runner::Game(g)) => match g.data {
            Some(GameSet::TikTakToes) => cases::game::tik_tak_toes::runner(),
            Some(GameSet::PaperScissorsRock) => cases::game::paper_scissors_rock::runner(),
            None => GameSet::iter()
                .for_each(|x| run_with(Some(Runner::Game(GameArgs { data: Some(x) })))),
        },
        Some(Runner::PlayGame(g)) => match g.data {
            Some(GameSet::TikTakToes) => {
                let game = TikTakToes;
                let mut network = Network1::new_from_game(&game, 10,10,None);
                play_game(game, &mut network);
            }
            Some(GameSet::PaperScissorsRock) => cases::game::paper_scissors_rock::runner(),
            None => panic!("we need to specify which game to play... this should also be done in clap not panic"),        
        },
        Some(Runner::Data(d)) => match d.data {
            Some(DataSet::Or) => cases::simple::or::runner(),
            Some(DataSet::And) => cases::simple::and::runner(),
            //lol andor orand
            Some(DataSet::AndOr) => cases::simple::or_and::runner(),
            None => DataSet::iter()
                .for_each(|x| run_with(Some(Runner::Data(DataArgs { data: Some(x) })))),
        },
        None => {
            run_with(Some(Runner::Game(GameArgs { data: None })));
            run_with(Some(Runner::Data(DataArgs { data: None })));
        }
    }
}
