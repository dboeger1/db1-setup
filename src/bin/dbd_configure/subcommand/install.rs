use crate::{
    error::Error,
    platform::Platform,
};


pub fn subcommand_install(platform: &Platform) -> Result<(), Error> {
    if let Some(install_packages) = platform.install_packages {
        return install_packages();
    }

    Ok(())
}
