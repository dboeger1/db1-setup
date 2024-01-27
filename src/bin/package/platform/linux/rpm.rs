use crate::{
    error::Error,
    values::{
        DIR_PROJECT_PACKAGES,
        DIR_PROJECT_ROOT,
        FILE_TAR,
    },
};
use dboeger1_dotfiles::CARGO_NAME;
use lazy_static::lazy_static;
use print_command::run_and_print;
use std::{
    fs::{
        copy,
        create_dir_all,
    },
    path::PathBuf,
    process::Command,
};


lazy_static! {
    static ref DIR_PACKAGES_RPM: PathBuf = DIR_PROJECT_PACKAGES.join("rpm");

    static ref DIR_RPMBUILD: PathBuf = DIR_PACKAGES_RPM.join("rpmbuild");
    static ref DIR_RPMBUILD_BUILD: PathBuf = DIR_RPMBUILD.join("BUILD");
    static ref DIR_RPMBUILD_BUILDROOT: PathBuf = DIR_RPMBUILD.join("BUILDROOT");
    static ref DIR_RPMBUILD_RPMS: PathBuf = DIR_RPMBUILD.join("RPMS");
    static ref DIR_RPMBUILD_SOURCES: PathBuf = DIR_RPMBUILD.join("SOURCES");
    static ref DIR_RPMBUILD_SPECS: PathBuf = DIR_RPMBUILD.join("SPECS");
    static ref DIR_RPMBUILD_SRPMS: PathBuf = DIR_RPMBUILD.join("SRPMS");

    static ref FILE_TAR_COPY: PathBuf = DIR_RPMBUILD_SOURCES.join(
        FILE_TAR.file_name().unwrap()
    );

    static ref FILE_SPEC: PathBuf = DIR_PROJECT_ROOT.join(
        format!("assets/package/rpm/{CARGO_NAME}.spec"),
    );
    static ref FILE_SPEC_COPY: PathBuf = DIR_RPMBUILD_SPECS.join(
        FILE_SPEC.file_name().unwrap()
    );
}


pub(crate) fn rpmbuild() -> Result<(), Error> {
    // Create rpmbuild directories.
    for directory in [
        DIR_RPMBUILD_BUILD.as_path(),
        DIR_RPMBUILD_BUILDROOT.as_path(),
        DIR_RPMBUILD_RPMS.as_path(),
        DIR_RPMBUILD_SOURCES.as_path(),
        DIR_RPMBUILD_SPECS.as_path(),
        DIR_RPMBUILD_SRPMS.as_path(),
    ] {
        create_dir_all(directory)
            .map_err(|error| Error {
                message: format!(
                    "Failed to create directory: {}",
                    directory.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            })?;
    }

    // Copy source archive.
    copy(
        FILE_TAR.as_path(),
        FILE_TAR_COPY.as_path(),
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "Failed to copy file: {} -> {}",
                    FILE_TAR.to_string_lossy(),
                    FILE_TAR_COPY.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(()),
        )?;

    // Copy spec file.
    copy(
        FILE_SPEC.as_path(),
        FILE_SPEC_COPY.as_path(),
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "Failed to copy file: {} -> {}",
                    FILE_SPEC.to_string_lossy(),
                    FILE_SPEC_COPY.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(()),
        )?;

    // Build RPM package.
    let mut rpmbuild_command = Command::new("rpmbuild");
    rpmbuild_command.args([
        format!(
            "--define=_topdir {}",
            DIR_RPMBUILD.to_string_lossy(),
        ),
        "-ba".to_string(),
        FILE_SPEC_COPY.to_string_lossy().to_string(),
    ]);

    run_and_print(&mut rpmbuild_command, false)
        .map_err(|error| Error {
            message: "Error running rpmbuild command.".to_string(),
            source: Some(Box::new(error)),
        })
}
