#[cfg(target_os = "linux")]
mod linux;


use crate::error::Error;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub(crate) use linux::PLATFORM;


pub(crate) struct Platform {
    // Install
    pub(crate) install_packages: fn() -> Result<(), Error>,

    // Neovim
    pub(crate) neovim_destination: Option<PathBuf>,
    pub(crate) neovim_source: Option<PathBuf>,

    // SSH
    pub(crate) configure_ssh: fn() -> Result<(), Error>,

    // tmux
    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux_destination: Option<PathBuf>,
    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux_source: Option<PathBuf>,
}
