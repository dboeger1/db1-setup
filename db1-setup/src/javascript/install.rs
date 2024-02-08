use crate::{
    error::Error,
    platform::javascript::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
