use crate::platform::{
    linux::{
        rpm::rpmbuild,
        tar::archive_sources_tar,
    },
    Platform,
};
use lazy_static::lazy_static;


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        archive_sources: archive_sources_tar,
        package: rpmbuild,
    };
}
