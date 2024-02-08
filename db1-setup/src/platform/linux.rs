mod dnf;
mod fedora;
mod rust;


use crate::{
    CARGO_NAME,
    OS_INFO,
    platform::Platform,
};
use lazy_static::lazy_static;
use os_info::Type;
use std::path::PathBuf;



lazy_static! {
    pub(super) static ref PLATFORM: Option<Platform> =
        match OS_INFO.os_type() {
            Type::Fedora => fedora::PLATFORM.clone(),
            _ => None,
        };

    pub(crate) static ref INSTALL_DIR: PathBuf =
        PathBuf::from(format!("/opt/{CARGO_NAME}"));
}
