use crate::{
    error::Error,
    platform::javascript::Platform,
};


pub(super) fn verify(platform: &Platform) -> Result<(), Error> {
    (platform.verify)()
}
