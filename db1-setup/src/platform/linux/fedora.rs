mod f38;
mod f39;


use crate::platform::Strategy;
use os_info::Version;


pub(super) fn strategy() -> Option<Strategy> {
    match os_info::get().version() {
        Version::Semantic(38, 0, 0) => Some(f38::STRATEGY.clone()),
        Version::Semantic(39, 0, 0) => Some(f39::STRATEGY.clone()),
        _ => None,
    }
}
