use crate::{
    error::Error,
    platform::hostname::Platform,
};


pub(super) fn verify(platform: &Platform) -> Result<(), Error> {
    (platform.verify)()
}
