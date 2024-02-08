use crate::{
    error::Error,
    platform::c::Platform,
};


pub(super) fn verify(platform: &Platform) -> Result<(), Error> {
    (platform.verify)()
}
