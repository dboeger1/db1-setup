use crate::{
    error::Error,
    platform::rust::Platform,
};


pub(super) fn verify(platform: &Platform) -> Result<(), Error> {
    (platform.verify)()
}
