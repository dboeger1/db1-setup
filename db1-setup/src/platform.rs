#[cfg(target_os = "linux")]
mod linux;


use crate::{
    error::Error,
    source_destination::SourceDestination,
};

#[cfg(target_os = "linux")]
pub(crate) use linux::PLATFORM;


pub(crate) struct Platform {
    pub(crate) configure_ssh: fn() -> Result<(), Error>,
    pub(crate) install_packages: fn() -> Result<(), Error>,
    pub(crate) neovim_paths: SourceDestination,
    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux_paths: SourceDestination,
}
