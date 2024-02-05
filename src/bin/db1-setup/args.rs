use clap::Parser;
use crate::subcommand::Subcommand;


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) subcommand: Subcommand,
}
