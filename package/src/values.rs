use const_format::concatcp;
use db1_setup::CARGO_NAME;
use std::{
    env::current_exe,
    path::PathBuf,
};


pub const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME_VERSION: &str = concatcp!(CARGO_NAME, "-", CARGO_VERSION);

pub(crate) fn dir_project_root() -> PathBuf {
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
    .to_path_buf()
}

pub(crate) fn file_gitignore() -> PathBuf {
    dir_project_root().join(".gitignore")
}

pub(crate) fn dir_project_git() -> PathBuf {
    dir_project_root().join(".git")
}
pub(crate) fn dir_project_target() -> PathBuf {
    dir_project_root().join("target")
}
pub(crate) fn dir_project_packages() -> PathBuf {
    dir_project_root().join("packages")
}

pub(crate) fn dir_packages_src() -> PathBuf {
    dir_project_packages().join("src")
}

pub(crate) fn file_tar() -> PathBuf {
    dir_packages_src().join(format!("{NAME_VERSION}.tar.gz"))
}
