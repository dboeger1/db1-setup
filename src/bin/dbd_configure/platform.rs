#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


use crate::{
    error::Error,
    source_destination::SourceDestination,
};


pub(crate) trait Platform: Sync {
    fn get_neovim_paths(&self) -> Option<SourceDestination> {
        None
    }

    fn get_tmux_paths(&self) -> Option<SourceDestination> {
        None
    }

    fn get_install_packages(&self) -> Option<fn() -> Result<(), Error>> {
        None
    }
}
