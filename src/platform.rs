#[cfg(target_os = "darwin")]
mod darwin;
#[cfg(target_os = "darwin")]
pub use darwin::*;


#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;


#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
