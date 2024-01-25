mod f38;
mod f39;


use crate::platform::Platform;
use dboeger1_dotfiles::OS_INFO;
use lazy_static::lazy_static;
use os_info::Version;


lazy_static! {
    pub(crate) static ref PLATFORM: Option<&'static Platform> =
        match OS_INFO.version() {
            Version::Semantic(38, 0, 0) => Some(&f38::PLATFORM),
            Version::Semantic(39, 0, 0) => Some(&f39::PLATFORM),
            _ => None,
        };
}
