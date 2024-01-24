#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


use crate::{
    error::ConfigureError,
    source_destination::SourceDestination,
};
use lazy_static::lazy_static;


pub(crate) trait Platform: Sync {
    fn get_neovim_paths(&self) -> Option<SourceDestination> {
        None
    }

    fn get_tmux_paths(&self) -> Option<SourceDestination> {
        None
    }

    fn get_install_packages(&self) -> Option<
            fn() -> Result<(), ConfigureError>
        > {
        None
    }
}

lazy_static! {
    pub(crate) static ref OS_INFO: os_info::Info = os_info::get();
}
