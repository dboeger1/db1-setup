mod fedora;
mod rpm;
mod tar;


use crate::platform::Platform;
use dboeger1_dotfiles::OS_INFO;
use lazy_static::lazy_static;
use os_info::Type;


lazy_static! {
    pub(crate) static ref PLATFORM: Option<&'static Platform> =
        match OS_INFO.os_type() {
            Type::Fedora => *fedora::PLATFORM,
            _ => None,
        };
}
