use crate::{
    error::Error,
    platform::utilities::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
