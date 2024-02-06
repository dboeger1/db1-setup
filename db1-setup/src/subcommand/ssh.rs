use crate::{
    error::Error,
    platform::Platform,
};


#[derive(clap::Args, PartialEq, Eq)]
pub(crate) struct Args {
    #[arg(short, long)]
    comment: Option<String>,

    #[arg(short, long)]
    force: bool,
}


pub(crate) fn subcommand_ssh(
    platform: &Platform,
    _args: &Args,
) -> Result<(), Error> {
    (platform.configure_ssh)()
}
