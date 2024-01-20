mod f38;
mod f39;


use crate::platform::{
    OS_INFO,
    Platform,
};
use os_info::Version;
use std::sync::Arc;


lazy_static! {
    pub(crate) static ref PLATFORM: Option<Arc<&'static dyn Platform>> =
        match OS_INFO.version() {
            Version::Semantic(38, 0, 0) => Some(Arc::new(&f38::PLATFORM)),
            Version::Semantic(39, 0, 0) => Some(Arc::new(&f39::PLATFORM)),
            _ => None,
        };
}
