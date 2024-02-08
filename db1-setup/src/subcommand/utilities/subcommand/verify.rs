use crate::{
    error::Error,
    platform::Platform,
};


pub(crate) fn subcommand_verify(platform: &Platform) -> Result<(), Error> {
    (platform.utilities.verify)()
}
