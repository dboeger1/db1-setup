#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


use crate::{
    error::ConfigureError,
    source_destination::SourceDestination,
};


pub(crate) trait Platform: Sync {
    fn get_neovim_paths(&self) -> Option<SourceDestination>;
    fn get_tmux_paths(&self) -> Option<SourceDestination>;
    fn install_packages(&self) -> Result<(), ConfigureError>;
}

lazy_static! {
    pub(crate) static ref OS_INFO: os_info::Info = os_info::get();
}
