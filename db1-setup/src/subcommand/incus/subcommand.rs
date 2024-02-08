mod install;
mod verify;


use crate::{
    error::Error,
    platform::Platform,
};
use install::subcommand_install;
use verify::subcommand_verify;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub(crate) enum Subcommand {
    Install,
    Verify,
}


pub(crate) fn execute_subcommand(
    platform: &Platform,
    subcommand: &Subcommand,
) -> Result<(), Error> {
    match subcommand {
        Subcommand::Install => subcommand_install(platform),
        Subcommand::Verify => subcommand_verify(platform),
    }
}
