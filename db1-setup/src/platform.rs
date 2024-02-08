#[cfg(target_os = "linux")]
mod linux;


use crate::{
    error::Error,
    subcommand::hostname::subcommand::configure::Args as HostnameConfigureArgs,
    subcommand::ssh::subcommand::configure::Args as SshConfigureArgs,
};
use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub(crate) use linux::PLATFORM;


pub(crate) struct Platform {
    // C
    pub(crate) c_install: fn() -> Result<(), Error>,
    pub(crate) c_verify: fn() -> Result<(), Error>,

    // C++
    pub(crate) cpp_install: fn() -> Result<(), Error>,
    pub(crate) cpp_verify: fn() -> Result<(), Error>,

    // Hostname
    pub(crate) hostname_configure: fn(
        &HostnameConfigureArgs,
    ) -> Result<(), Error>,
    pub(crate) hostname_verify: fn() -> Result<(), Error>,

    // Incus
    #[cfg(target_os = "linux")]
    pub(crate) incus_install: fn() -> Result<(), Error>,
    #[cfg(target_os = "linux")]
    pub(crate) incus_verify: fn() -> Result<(), Error>,

    // Javascript
    pub(crate) javascript_install: fn() -> Result<(), Error>,
    pub(crate) javascript_verify: fn() -> Result<(), Error>,

    // Neovim
    pub(crate) neovim_destination: Option<PathBuf>,
    pub(crate) neovim_source: Option<PathBuf>,

    pub(crate) neovim_install: fn() -> Result<(), Error>,
    pub(crate) neovim_verify: fn() -> Result<(), Error>,

    // RPM
    #[cfg(target_os = "linux")]
    pub(crate) rpm_install: fn() -> Result<(), Error>,
    #[cfg(target_os = "linux")]
    pub(crate) rpm_verify: fn() -> Result<(), Error>,

    // Rust
    pub(crate) rust_install: fn() -> Result<(), Error>,
    pub(crate) rust_verify: fn() -> Result<(), Error>,

    // SSH
    pub(crate) ssh_configure: fn(
        &SshConfigureArgs,
    ) -> Result<(), Error>,
    pub(crate) ssh_verify: fn() -> Result<(), Error>,

    // tmux
    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux_install: fn() -> Result<(), Error>,
    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux_verify: fn() -> Result<(), Error>,

    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux_destination: Option<PathBuf>,
    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux_source: Option<PathBuf>,

    // Utilities
    pub(crate) utilities_install: fn() -> Result<(), Error>,
    pub(crate) utilities_verify: fn() -> Result<(), Error>,
}
