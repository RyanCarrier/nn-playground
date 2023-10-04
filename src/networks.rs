use clap::ValueEnum;
use strum_macros::{Display, EnumIter};

pub mod network1;
pub mod network2;
#[derive(Debug, Clone, EnumIter, ValueEnum, Display)]
pub enum Networks {
    Network1,
    Network2,
}
