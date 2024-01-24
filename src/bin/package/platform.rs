#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


use lazy_static::lazy_static;


pub(crate) trait Platform: Sync {
    fn stuff(&self);
}

lazy_static! {
    pub(crate) static ref OS_INFO: os_info::Info = os_info::get();
}
