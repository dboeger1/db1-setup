mod configure;
mod install;
mod verify;


use configure::configure;
use crate::{
    error::Error,
    platform::neovim::Platform,
};
use install::install;
use verify::verify;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    Configure(configure::Args),
    Install,
    Verify,
}

#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}


pub(super) fn execute(platform: &Platform, args: &Args) -> Result<(), Error> {
    match &args.subcommand {
        Subcommand::Configure(args) => configure(platform, &args),
        Subcommand::Install => install(platform),
        Subcommand::Verify => verify(platform),
    }
}
