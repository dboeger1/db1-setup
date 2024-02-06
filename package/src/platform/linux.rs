mod fedora;
mod rpm;
mod tar;


use common::OS_INFO;
use crate::platform::Platform;
use lazy_static::lazy_static;
use os_info::Type;


lazy_static! {
    pub(crate) static ref PLATFORM: Option<&'static Platform> =
        match OS_INFO.os_type() {
            Type::Fedora => *fedora::PLATFORM,
            _ => None,
        };
}
