use crate::platform::{
    linux::{
        rpm::rpmbuild,
        tar::archive_sources_tar,
    },
    Platform,
};


pub(crate) const PLATFORM: Platform = Platform {
    archive_sources: archive_sources_tar,
    package: rpmbuild,
};
