use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_install(platform: &Platform) -> Result<(), Error> {
    (platform.utilities.install)()
}
