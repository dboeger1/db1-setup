use crate::{
    error::Error,
    platform::tmux::Platform,
};


pub(super) fn install(platform: &Platform) -> Result<(), Error> {
    (platform.install)()
}
