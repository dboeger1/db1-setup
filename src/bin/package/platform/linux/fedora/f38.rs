use crate::platform::{
    linux::tar_sources,
    Platform,
};
use lazy_static::lazy_static;


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        archive_sources: tar_sources
    };
}
