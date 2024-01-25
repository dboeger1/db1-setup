#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


pub(crate) struct Platform {
    pub(crate) stuff: fn (),
}
