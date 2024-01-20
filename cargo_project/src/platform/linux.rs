mod fedora;


use crate::platform::{
    OS_INFO,
    Platform,
};
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
}
