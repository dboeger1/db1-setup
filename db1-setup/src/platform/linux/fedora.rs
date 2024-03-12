mod f38;
mod f39;


use crate::{
    OS_INFO,
    platform::Strategy,
};
use lazy_static::lazy_static;
use os_info::Version;


lazy_static! {
    pub(super) static ref STRATEGY: Option<Strategy> =
        match OS_INFO.version() {
            Version::Semantic(38, 0, 0) => Some(f38::STRATEGY.clone()),
            Version::Semantic(39, 0, 0) => Some(f39::STRATEGY.clone()),
            _ => None,
        };
}
