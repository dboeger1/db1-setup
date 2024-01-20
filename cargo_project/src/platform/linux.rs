mod fedora;
mod ubuntu;


use crate::platform::{
    OS_INFO,
    Platform,
};
use std::{
    env::var,
    path::PathBuf,
};



lazy_static! {
    pub static ref PLATFORM: Option<dyn Platform> = {
        match OS_INFO.os_type() {
            os_info::Type::Fedora => fedora::get_platform(),
            os_info::Type::Debian => ubuntu::get_platform(),
            _ => None,
        }
    };
    // User paths.
    pub static ref HOME_DIR: PathBuf =
        PathBuf::from(var("HOME").unwrap());
}
