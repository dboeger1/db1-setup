use crate::{
    error::Error,
    platform::hostname::Platform,
};


#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[arg(short, long)]
    pub force: bool,
}


pub(super) fn configure(platform: &Platform, args: &Args) -> Result<(), Error> {
    (platform.configure)(args)
}
