mod fedora;


use crate::{
    CARGO_NAME,
    platform::{
        OS_INFO,
        Platform,
    },
};
use lazy_static::lazy_static;
use os_info::Type;
use std::{
    env::var,
    path::PathBuf, 
    sync::Arc,
};



lazy_static! {
    pub(crate) static ref PLATFORM: Option<Arc<&'static dyn Platform>> =
        match OS_INFO.os_type() {
            Type::Fedora => fedora::PLATFORM.clone(),
            _ => None,
        };

    // User paths.
    pub(crate) static ref HOME_DIR: PathBuf =
        PathBuf::from(var("HOME").unwrap());
    pub(crate) static ref INSTALL_DIR: PathBuf =
        PathBuf::from(format!("/opt/{}", CARGO_NAME));
}
