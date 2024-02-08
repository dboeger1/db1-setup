use crate::{
    error::Error,
    values::{
        DIR_PACKAGES_SRC,
        DIR_PROJECT_GIT,
        DIR_PROJECT_PACKAGES,
        DIR_PROJECT_ROOT,
        DIR_PROJECT_TARGET,
        FILE_GITIGNORE,
        FILE_TAR,
        NAME_VERSION,
    },
};
use prun::prun;
use std::{
    fs::create_dir_all,
    process::{
        Command,
        Output,
    },
};


pub(crate) fn archive_sources_tar() -> Result<(), Error> {
    create_dir_all(DIR_PACKAGES_SRC.as_path())
        .map_err(|error| Error {
            message: format!(
                "Failed to create directory: {}",
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
            FILE_GITIGNORE
                .file_name()
                .unwrap()
                .to_string_lossy(),
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
        format!("--transform=s#^.#{NAME_VERSION}#"),
        format!(
            "--file={}",
            FILE_TAR.to_string_lossy(),
        ),
        ".".to_string(),
    ]);

    let output: Output;
    match prun(&mut tar_command, false) {
        Ok(output_inner) => output = output_inner,
        Err(error) => return Err(Error {
            message: "Error running tar command.".to_string(),
            source: Some(Box::new(error)),
        }),
    }

    if !output.status.success() {
        return Err(Error {
            message: "tar command failed.".to_string(),
            source: None,
        });
    };

    Ok(())
}
