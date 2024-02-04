use crate::{
    error::Error,
    platform::Platform,
};


pub fn subcommand_ssh(platform: &Platform) -> Result<(), Error> {
    if let Some(configure_ssh) = platform.configure_ssh {
        return configure_ssh();
    }

    Ok(())
}
