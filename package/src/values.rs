use const_format::concatcp;
use db1_setup::CARGO_NAME;
use lazy_static::lazy_static;
use std::{
    env::current_exe,
    path::PathBuf,
};


pub const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME_VERSION: &str = concatcp!(CARGO_NAME, "-", CARGO_VERSION);

lazy_static! {
    pub(crate) static ref DIR_PROJECT_ROOT: PathBuf =
        current_exe()
        .unwrap()
        // binary's parent directory
        .parent()
        .unwrap()
        // walk up the ancestors
        .ancestors()
        // find ancestor directory containing Cargo.toml
        .find(|path| {
            path
                // get contents of ancestor directory
                .read_dir()
                .unwrap()
                .map(|entry| entry.unwrap())
                // find Cargo.toml
                .find(|entry| entry.path().ends_with("Cargo.toml"))
                .is_some()
        })
        .unwrap()
        .to_path_buf();

    pub(crate) static ref DIR_PROJECT_GIT: PathBuf =
        DIR_PROJECT_ROOT.join(".git");
    pub(crate) static ref DIR_PROJECT_TARGET: PathBuf =
        DIR_PROJECT_ROOT.join("target");
    pub(crate) static ref DIR_PROJECT_PACKAGES: PathBuf =
        DIR_PROJECT_ROOT.join("packages");

    pub(crate) static ref DIR_PACKAGES_SRC: PathBuf =
        DIR_PROJECT_PACKAGES.join("src");

    pub(crate) static ref FILE_TAR: PathBuf =
        DIR_PACKAGES_SRC.join(format!("{NAME_VERSION}.tar.gz"));
}
