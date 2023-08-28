use crate::{
    FILES_CSV_NAME,
    PROJECT_ASSETS_DIR,
    PROJECT_PACKAGES_DIR,
};
use std::path::PathBuf;


pub const PLATFORM_NAME: &str = "windows";

lazy_static! {
    // Asset paths.
    pub static ref ASSETS_DIR: PathBuf =
        PROJECT_ASSETS_DIR.join(PLATFORM_NAME);
    pub static ref ASSETS_FILES_CSV_FILE: PathBuf =
        ASSETS_DIR.join(FILES_CSV_NAME);

    // Package paths.
    pub static ref PACKAGES_DIR: PathBuf =
        PROJECT_PACKAGES_DIR.join(PLATFORM_NAME);

    // Install paths.
}
