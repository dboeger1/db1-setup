mod f38;
mod f39;


use crate::platform::Platform;
use os_info::Version;


pub(crate) const PLATFORM: Option<Platform> =
    match os_info::get().version() {
        Version::Semantic(38, 0, 0) => Some(f38::PLATFORM),
        Version::Semantic(39, 0, 0) => Some(f39::PLATFORM),
        _ => None,
    };
