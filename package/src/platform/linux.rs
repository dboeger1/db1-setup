mod fedora;
mod rpm;
mod tar;


use crate::platform::Platform;
use os_info::Type;


pub(crate) const PLATFORM: Option<Platform> =
    match os_info::get().os_type() {
        Type::Fedora => fedora::PLATFORM,
        _ => None,
    };
