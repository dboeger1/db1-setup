mod f38;
mod f39;


use crate::{
    OS_INFO,
    platform::Platform,
};
use lazy_static::lazy_static;
use os_info::Version;


lazy_static! {
    pub(super) static ref PLATFORM: Option<Platform> =
        match OS_INFO.version() {
            Version::Semantic(38, 0, 0) => Some(f38::PLATFORM.clone()),
            Version::Semantic(39, 0, 0) => Some(f39::PLATFORM.clone()),
            _ => None,
        };
}
