use crate::{
    error::Error,
    values::{
        dir_project_packages,
        dir_project_root,
        file_tar,
    },
};
use db1_setup::CARGO_NAME;
use prun::prun;
use std::{
    fs::{
        copy,
        create_dir_all,
    },
    path::PathBuf,
    process::{
        Command,
        Output,
    },
};


fn dir_packages_rpm() -> PathBuf {dir_project_packages().join("rpm")}

fn dir_rpmbuild() -> PathBuf {dir_packages_rpm().join("rpmbuild")}
fn dir_rpmbuild_build() -> PathBuf {dir_rpmbuild().join("build")}
fn dir_rpmbuild_buildroot() -> PathBuf {dir_rpmbuild().join("buildroot")}
fn dir_rpmbuild_rpms() -> PathBuf {dir_rpmbuild().join("rpms")}
fn dir_rpmbuild_sources() -> PathBuf {dir_rpmbuild().join("sources")}
fn dir_rpmbuild_specs() -> PathBuf {dir_rpmbuild().join("specs")}
fn dir_rpmbuild_srpms() -> PathBuf {dir_rpmbuild().join("srpms")}

fn file_tar_copy() -> PathBuf {dir_rpmbuild_sources().join(
    file_tar().file_name().unwrap()
)}

fn file_spec() -> PathBuf {dir_project_root().join(
    format!("assets/package/rpm/{CARGO_NAME}.spec"),
)}
fn file_spec_copy() -> PathBuf {dir_rpmbuild_specs().join(
    file_spec().file_name().unwrap()
)}


pub(crate) fn rpmbuild() -> Result<(), Error> {
    // Create rpmbuild directories.
    for directory in [
        dir_rpmbuild_build().as_path(),
        dir_rpmbuild_buildroot().as_path(),
        dir_rpmbuild_rpms().as_path(),
        dir_rpmbuild_sources().as_path(),
        dir_rpmbuild_specs().as_path(),
        dir_rpmbuild_srpms().as_path(),
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
        file_tar().as_path(),
        file_tar_copy().as_path(),
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "Failed to copy file: {} -> {}",
                    file_tar().to_string_lossy(),
                    file_tar_copy().to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(()),
        )?;

    // Copy spec file.
    copy(
        file_spec().as_path(),
        file_spec_copy().as_path(),
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "Failed to copy file: {} -> {}",
                    file_spec().to_string_lossy(),
                    file_spec_copy().to_string_lossy(),
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
            dir_rpmbuild().to_string_lossy(),
        ),
        "-ba".to_string(),
        file_spec_copy().to_string_lossy().to_string(),
    ]);

    let output: Output;
    match prun(&mut rpmbuild_command, false) {
        Ok(output_inner) => output = output_inner,
        Err(error) => return Err(Error {
            message: "Error running rpmbuild command.".to_string(),
            source: Some(Box::new(error)),
        }),
    }

    if !output.status.success() {
        return Err(Error {
            message: "rpmbuild command failed.".to_string(),
            source: None,
        });
    };

    Ok(())
}
