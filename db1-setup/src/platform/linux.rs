mod fedora;


use crate::{
    CARGO_NAME,
    platform::Strategy,
};
use os_info::Type;
use std::path::PathBuf;


pub(crate) fn install_dir() -> PathBuf {
    PathBuf::from(format!("/opt/{CARGO_NAME}"))
}

pub(super) fn strategy() -> Option<Strategy> {
    match os_info::get().os_type() {
        Type::Fedora => fedora::strategy().clone(),
        _ => None,
    }
}
