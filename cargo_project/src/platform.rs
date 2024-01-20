#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


use crate::{
    error::ConfigureError,
    source_destination::SourceDestination,
};
use std::fmt::Debug;


pub(crate) trait Platform: Debug + Sync {
    fn get_neovim_paths(&self) -> SourceDestination;
    fn get_tmux_paths(&self) -> SourceDestination;
    fn install_packages(&self) -> Result<(), ConfigureError>;
}

lazy_static! {
    pub(crate) static ref OS_INFO: os_info::Info = os_info::get();
}
