use crate::{
    error::Error,
    platform::Platform,
};


pub fn subcommand_install(platform: &Platform) -> Result<(), Error> {
    (platform.install_packages)()
}
