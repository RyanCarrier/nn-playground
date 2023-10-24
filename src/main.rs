use clap::{Args, Parser, Subcommand};
use nn_playground::{
    cases::{
        self,
        game::{play::play_game, tik_tak_toes::TikTakToes},
    },
    networks::{network1::network::Network1, Networks},
    traits::{generic_game_case::GenericGameCase, network_traits::BaseNetwork},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Parser, Debug)]
// #[command(name = "nn")]
struct Cli {
    #[command(subcommand)]
    runner: Option<Runner>,
    #[clap(short, long)]
    network: Option<Networks>,
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
    TikTakToesTest,
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
    Xor,
}

fn main() {
    let cli = Cli::parse();
    run_with(cli.runner, &cli.network);
}

fn run_with(runner: Option<Runner>, network: &Option<Networks>) {
    match runner {
        Some(Runner::Game(g)) => match g.data {
            Some(GameSet::TikTakToes) => cases::game::tik_tak_toes::runner(),
            Some(GameSet::TikTakToesTest) => cases::game::tik_tak_toes::game_test(),
            Some(GameSet::PaperScissorsRock) => cases::game::paper_scissors_rock::runner(),
            None => GameSet::iter()
                .for_each(|x| run_with(Some(Runner::Game(GameArgs { data: Some(x) })),network)),
        },
        Some(Runner::PlayGame(g)) => match g.data {
            Some(GameSet::TikTakToes) => {
                let game = TikTakToes;
                let mut network = Network1::new(game.input_nodes(),game.output_nodes(), 10,10,None);
                play_game(game, &mut network);
            }
            Some(GameSet::TikTakToesTest) => panic!("not implemented"),
            Some(GameSet::PaperScissorsRock) => cases::game::paper_scissors_rock::runner(),
            None => panic!("we need to specify which game to play... this should also be done in clap not panic"),        
        },
        Some(Runner::Data(d)) => match d.data {
            Some(DataSet::Or) => cases::simple::or::runner(network),
            Some(DataSet::And) => cases::simple::and::runner(network),
            //lol andor orand
            Some(DataSet::AndOr) => cases::simple::or_and::runner(network),
            Some(DataSet::Xor) => cases::simple::xor::runner(network),
            None => DataSet::iter()
                .for_each(|x| run_with(Some(Runner::Data(DataArgs { data: Some(x) })),network)),
        },
        None => {
            run_with(Some(Runner::Game(GameArgs { data: None })),network);
            run_with(Some(Runner::Data(DataArgs { data: None })),network);
        }
    }
}
