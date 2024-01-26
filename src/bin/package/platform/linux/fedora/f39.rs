use crate::platform::{
    linux::{
        rpm::rpmbuild,
        tar::tar_sources,
    },
    Platform,
};
use lazy_static::lazy_static;


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        archive_sources: tar_sources,
        package: rpmbuild,
    };
}
