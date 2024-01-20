#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;


use crate::{
    error::ConfigureError,
    source_destination::SourceDestination,
};


pub(crate) trait Platform {
    fn get_neovim_paths() -> SourceDestination;
    fn get_tmux_paths() -> SourceDestination;

    fn install_packages() -> Result<(), ConfigureError>;
}

lazy_static! {
    pub(crate) static ref OS_INFO: os_info::Info = os_info::get();
}
