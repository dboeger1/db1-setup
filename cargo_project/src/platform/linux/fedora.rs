mod f38;
mod f39;


use crate::platform::{
    OS_INFO,
    Platform,
};
use os_info::Version;


pub(crate) fn get_platform() -> Option<dyn Platform> {
    match OS_INFO.version() {
        Version::Semantic(38, 0, 0) => Some(f38::get_platform()),
        Version::Semantic(39, 0, 0) => Some(f39::get_platform()),
        _ => None,
    }
}
