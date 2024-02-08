mod install;
mod verify;


use crate::{
    error::Error,
    platform::c::Platform,
};
use install::install;
use verify::verify;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    Install,
    Verify,
}

#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}


pub(super) fn execute(
    platform: &Platform,
    args: &Args,
) -> Result<(), Error> {
    match &args.subcommand {
        Subcommand::Install => install(platform),
        Subcommand::Verify => verify(platform),
    }
}
