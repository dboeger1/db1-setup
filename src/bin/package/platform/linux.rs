mod fedora;


use crate::platform::{
    OS_INFO,
    Platform,
};
use lazy_static::lazy_static;
use os_info::Type;
use std::sync::Arc;


lazy_static! {
    pub(crate) static ref PLATFORM: Option<Arc<&'static dyn Platform>> =
        match OS_INFO.os_type() {
            Type::Fedora => fedora::PLATFORM.clone(),
            _ => None,
        };
}
