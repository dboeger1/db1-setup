use crate::{
    error::Error,
    platform::tmux::Platform,
};


pub(super) fn verify(platform: &Platform) -> Result<(), Error> {
    (platform.verify)()
}
