mod platform;

pub use platform::*;

use std::{
    env::current_exe,
    path::PathBuf,
};

#[macro_use]
extern crate lazy_static;


// Basic information.
pub const CARGO_NAME: &str = env!("CARGO_PKG_NAME");
pub const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");

// Asset paths.
pub const FILES_CSV_NAME: &str = "files.csv";

// Static references computed at runtime.
lazy_static! {
    // Basic information.
    pub static ref NAME_VERSION: String =
        format!("{}-{}", CARGO_NAME, CARGO_VERSION);

    // Project root path.
    pub static ref PROJECT_ROOT_DIR: PathBuf =
        current_exe()
        .unwrap()
        // binary's parent directory
        .parent()
        .unwrap()
        // walk up the ancestors
        .ancestors()
        // find ancestor containing Cargo.toml
        .find(|path| {
            path
                // get contents of ancestor
                .read_dir()
                .unwrap()
                .map(|entry| entry.unwrap())
                // find Cargo.toml
                .find(|entry| entry.path().ends_with("Cargo.toml"))
                .is_some()
        })
        .unwrap()
        .to_path_buf();

    // Asset paths.
    pub static ref PROJECT_ASSETS_DIR: PathBuf =
        PROJECT_ROOT_DIR.join("assets");
    pub static ref PROJECT_ASSETS_PLATFORM_DIR: PathBuf =
        PROJECT_ASSETS_DIR.join("platform");

    pub static ref PROJECT_ASSETS_NEOVIM_DIR: PathBuf =
        PROJECT_ASSETS_DIR.join("neovim");

    pub static ref PROJECT_ASSETS_TMUX_DIR: PathBuf =
        PROJECT_ASSETS_DIR.join("tmux");

    pub static ref PROJECT_ASSETS_DEB_DIR: PathBuf =
        PROJECT_ASSETS_DIR.join("deb");
    pub static ref PROJECT_ASSETS_DEB_CONTROL_FILE: PathBuf =
        PROJECT_ASSETS_DEB_DIR.join("control");

    pub static ref PROJECT_ASSETS_RPM_DIR: PathBuf =
        PROJECT_ASSETS_DIR.join("rpm");
    pub static ref PROJECT_ASSETS_RPM_SPEC_FILE: PathBuf =
        PROJECT_ASSETS_RPM_DIR.join("name.spec");

    // Package paths.
    pub static ref PROJECT_PACKAGES_DIR: PathBuf =
        PROJECT_ROOT_DIR.join("packages");
}
