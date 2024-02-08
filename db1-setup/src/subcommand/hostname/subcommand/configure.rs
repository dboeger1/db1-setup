use crate::{
    error::Error,
    platform::Platform,
};


#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[arg(short, long)]
    pub force: bool,
}


pub(crate) fn subcommand_configure(
    platform: &Platform,
    args: &Args,
) -> Result<(), Error> {
    (platform.hostname_configure)(args)
}
