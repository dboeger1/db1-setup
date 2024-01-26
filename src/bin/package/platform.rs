#[cfg(target_os = "linux")]
mod linux;


use crate::error::Error;

#[cfg(target_os = "linux")]
pub(crate) use linux::*;


pub(crate) struct Platform {
    pub(crate) archive_sources: fn () -> Result<(), Error>,
    pub(crate) package: fn () -> Result<(), Error>,
}
