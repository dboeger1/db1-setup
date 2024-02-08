pub mod configure;
mod verify;


use configure::configure;
use crate::{
    error::Error,
    platform::ssh::Platform,
};
use verify::verify;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    Configure(configure::Args),
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
        Subcommand::Verify => verify(platform),
    }
}
