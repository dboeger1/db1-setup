use crate::{
    error::Error,
    platform::incus::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
