use crate::{
    error::Error,
    platform::neovim::Platform,
};

pub(super) fn verify(platform: &Platform) -> Result<(), Error> {
    (platform.verify)()
}
