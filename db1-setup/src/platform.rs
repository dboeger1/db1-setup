#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "darwin")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;


use crate::Error;
use std::{
    env::var,
    path::PathBuf,
};


pub(crate) fn home_dir() -> PathBuf {
    PathBuf::from(var("HOME").unwrap())
}


pub(crate) type Strategy = fn () -> Result<(), Error>;

#[cfg(target_os = "linux")]
pub(super) fn strategy() -> Option<Strategy> {
    linux::strategy().clone()
}
#[cfg(target_os = "darwin")]
pub(super) fn strategy() -> Option<Strategy> {
    macos::strategy().clone()
}
#[cfg(target_os = "windows")]
pub(super) fn strategy() -> Option<Strategy> {
    windows::strategy().clone()
}
