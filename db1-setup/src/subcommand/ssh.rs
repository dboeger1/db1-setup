use crate::{
    error::Error,
    platform::Platform,
};


#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[arg(short, long)]
    pub comment: Option<String>,

    #[arg(short, long)]
    pub force: bool,
}


pub(crate) fn subcommand_ssh(
    platform: &Platform,
    _args: &Args,
) -> Result<(), Error> {
    (platform.configure_ssh)()
}
