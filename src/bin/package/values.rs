use lazy_static::lazy_static;
use std::{
    env::current_exe,
    path::PathBuf,
};


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

    pub(crate) static ref DIR_PACKAGES: PathBuf =
        DIR_PROJECT_ROOT.join("packages");
}
