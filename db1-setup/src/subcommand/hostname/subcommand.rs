mod configure;
mod verify;


use configure::subcommand_configure;
use crate::{
    error::Error,
    platform::Platform,
};
use verify::subcommand_verify;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub(crate) enum Subcommand {
    Configure,
    Verify,
}


pub(crate) fn execute_subcommand(
    platform: &Platform,
    args_subcommand: &Subcommand,
) -> Result<(), Error> {
    match args_subcommand {
        Subcommand::Configure => subcommand_configure(platform),
        Subcommand::Verify => subcommand_verify(platform),
    }
}
