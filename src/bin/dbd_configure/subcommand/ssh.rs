mod args;


use crate::{
    error::Error,
    platform::Platform,
};

pub(crate) use args::Args;


pub fn subcommand_ssh(
    platform: &Platform,
    _args: &Args,
) -> Result<(), Error> {
    if let Some(configure_ssh) = platform.configure_ssh {
        return configure_ssh();
    }

    Ok(())
}
