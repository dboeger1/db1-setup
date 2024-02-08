mod configure;
mod install;
mod verify;


use configure::subcommand_configure;
use crate::{
    error::Error,
    platform::Platform,
};
use install::subcommand_install;
use verify::subcommand_verify;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub(crate) enum Subcommand {
    Configure(configure::Args),
    Install,
    Verify,
}


pub(crate) fn execute_subcommand(
    platform: &Platform,
    subcommand: &Subcommand,
) -> Result<(), Error> {
    match subcommand {
        Subcommand::Configure(args) => subcommand_configure(platform, &args),
        Subcommand::Install => subcommand_install(platform),
        Subcommand::Verify => subcommand_verify(platform),
    }
}
