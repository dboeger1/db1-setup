#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "darwin")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;


use crate::Error;
use lazy_static::lazy_static;


#[cfg(target_os = "linux")]
lazy_static! {
    pub(super) static ref STRATEGY: Option<Strategy> =
        linux::STRATEGY.clone();
}

#[cfg(target_os = "darwin")]
lazy_static! {
    pub(super) static ref STRATEGY: Option<Strategy> =
        macos::STRATEGY.clone();
}

#[cfg(target_os = "windows")]
lazy_static! {
    pub(super) static ref STRATEGY: Option<Strategy> =
        windows::STRATEGY.clone();
}

pub(crate) type Strategy = fn() -> Result<(), Error>;
