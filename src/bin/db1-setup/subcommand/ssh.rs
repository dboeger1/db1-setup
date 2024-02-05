mod args;


use crate::{
    error::Error,
    platform::Platform,
};

pub(crate) use args::Args;


pub(crate) fn subcommand_ssh(
    platform: &Platform,
    _args: &Args,
) -> Result<(), Error> {
    (platform.configure_ssh)()
}
