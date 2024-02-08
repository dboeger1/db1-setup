use crate::{
    error::Error,
    platform::c::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
