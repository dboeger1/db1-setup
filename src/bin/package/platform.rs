#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::*;


pub(crate) trait Platform: Sync {
    fn stuff(&self);
}
