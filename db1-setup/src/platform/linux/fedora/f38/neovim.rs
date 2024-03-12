mod configure;
mod install;
mod verify;


use clap::Parser;
use crate::Error;
use configure::configure;
use install::install;
use verify::verify;


pub(super) fn execute(args: Args) -> Result<(), Error> {
    match args.subcommand {
        Subcommand::Configure(args) => configure(&args),
        Subcommand::Install => install(),
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
    Configure(configure::Args),
    Install,
    Verify,
}


//neovim
