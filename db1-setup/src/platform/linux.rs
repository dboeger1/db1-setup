mod dnf;
mod fedora;


use crate::{
    CARGO_NAME,
    OS_INFO,
    platform::Strategy,
};
use lazy_static::lazy_static;
use os_info::Type;
use std::path::PathBuf;


lazy_static! {
    pub(super) static ref STRATEGY: Option<Strategy> =
        match OS_INFO.os_type() {
            Type::Fedora => fedora::STRATEGY.clone(),
            _ => None,
        };

    pub(crate) static ref INSTALL_DIR: PathBuf =
        PathBuf::from(format!("/opt/{CARGO_NAME}"));
}
