use crate::{
    error::Error,
    platform::cpp::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
