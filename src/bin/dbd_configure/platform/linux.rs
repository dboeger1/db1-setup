mod fedora;


use crate::{
    CARGO_NAME,
    platform::Platform,
};
use dboeger1_dotfiles::OS_INFO;
use lazy_static::lazy_static;
use os_info::Type;
use std::path::PathBuf;



lazy_static! {
    pub(crate) static ref PLATFORM: Option<&'static Platform> =
        match OS_INFO.os_type() {
            Type::Fedora => *fedora::PLATFORM,
            _ => None,
        };

    // User paths.
    pub(crate) static ref INSTALL_DIR: PathBuf =
        PathBuf::from(format!("/opt/{}", CARGO_NAME));
}
