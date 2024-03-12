mod configure;
mod verify;


use clap::Parser;
use crate::Error;
use configure::configure;
use verify::verify;


pub(super) fn execute(args: Args) -> Result<(), Error> {
    match args.subcommand {
        Subcommand::Configure => configure(),
        Subcommand::Verify => verify(),
    }
}

#[derive(Parser, PartialEq, Eq)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    Configure,
    Verify,
}
