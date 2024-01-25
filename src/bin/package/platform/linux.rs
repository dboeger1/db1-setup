mod fedora;


use crate::{
    error::Error,
    platform::Platform,
    values::{
        DIR_PACKAGES_SRC,
        DIR_PROJECT_GIT,
        DIR_PROJECT_PACKAGES,
        DIR_PROJECT_ROOT,
        DIR_PROJECT_TARGET,
        FILE_TAR,
        NAME_VERSION,
    },
};
use dboeger1_dotfiles::OS_INFO;
use lazy_static::lazy_static;
use os_info::Type;
use print_command::run_and_print;
use std::{
    fs::create_dir_all,
    process::Command,
};


lazy_static! {
    pub(crate) static ref PLATFORM: Option<&'static Platform> =
        match OS_INFO.os_type() {
            Type::Fedora => *fedora::PLATFORM,
            _ => None,
        };
}


pub(crate) fn tar_sources() -> Result<(), Error> {
    create_dir_all(DIR_PACKAGES_SRC.as_path())
        .map_err(|error| Error {
            message: format!(
                "failed to create directory: \"{}\"",
                DIR_PACKAGES_SRC.to_string_lossy(),
            ),
            source: Some(Box::new(error)),
        })?;

    let mut tar_command = Command::new("tar");
    tar_command.args([
        "--create".to_string(),
        "--gzip".to_string(),
        format!(
            "--directory={}",
            DIR_PROJECT_ROOT.to_string_lossy(),
        ),
        format!(
            "--exclude={}",
            DIR_PROJECT_GIT
                .file_name()
                .unwrap()
                .to_string_lossy(),
        ),
        format!(
            "--exclude={}",
            DIR_PROJECT_TARGET
                .file_name()
                .unwrap()
                .to_string_lossy(),
        ),
        format!(
            "--exclude={}",
            DIR_PROJECT_PACKAGES
                .file_name()
                .unwrap()
                .to_string_lossy(),
        ),
        format!(
            "--transform=s#^.#{}#",
            NAME_VERSION,
        ),
        format!(
            "--file={}",
            FILE_TAR.to_string_lossy(),
        ),
        ".".to_string(),
    ]);

    run_and_print(&mut tar_command, false)
        .map_err(|error| Error {
            message: "command failed".to_string(),
            source: Some(Box::new(error)),
        })
}
