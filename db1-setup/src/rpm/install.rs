use crate::{
    error::Error,
    platform::rpm::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
