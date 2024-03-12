mod install;
mod verify;


use clap::Parser;
use crate::Error;
use install::install;
use verify::verify;


pub(super) fn execute(args: Args) -> Result<(), Error> {
    match args.subcommand {
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
    Install,
    Verify,
}


//rpm-build
//rpm-devel
//rpmdevtools
//rpmlint
