use crate::{
    error::Error,
    platform::neovim::Platform,
};

pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
