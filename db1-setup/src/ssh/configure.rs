use crate::{
    error::Error,
    platform::ssh::Platform,
};


#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[arg(short, long)]
    pub comment: Option<String>,

    #[arg(short, long)]
    pub force: bool,
}


pub(super) fn configure(platform: &Platform, args: &Args) -> Result<(), Error> {
    (platform.configure)(args)
}
