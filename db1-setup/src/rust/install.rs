use crate::{
    error::Error,
    platform::rust::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
