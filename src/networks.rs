use clap::ValueEnum;
use strum_macros::{Display, EnumIter};

pub mod activation_functions;
pub mod network1;
pub mod network2;
pub mod network3;
#[derive(Debug, Clone, EnumIter, ValueEnum, Display)]
pub enum Networks {
    Network1,
    Network2,
    Network3,
}
