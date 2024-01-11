use crate::package_error::PackageError;
use dboeger1_dotfiles::*;
use std::{
    fs::{
        File,
        copy,
        create_dir_all,
    },
    process::{
        Command,
        Stdio,
    },
    str::from_utf8,
};


pub(crate) fn build_rpm() -> Result<(), PackageError> {
    for dir in [
        PACKAGES_RPMBUILD_BUILD_DIR.as_path(),
        PACKAGES_RPMBUILD_BUILDROOT_DIR.as_path(),
        PACKAGES_RPMBUILD_RPMS_DIR.as_path(),
        PACKAGES_RPMBUILD_SOURCES_DIR.as_path(),
        PACKAGES_RPMBUILD_SPECS_DIR.as_path(),
        PACKAGES_RPMBUILD_SRPMS_DIR.as_path(),
    ] {
        create_dir_all(dir)
            .map_err(|error| PackageError {
                message: format!(
                    "failed to create rpm directory \"{}\"",
                    dir.to_string_lossy(),
                ),
                source: Some(error),
            })?;
    }

    copy(
        PACKAGES_SRC_FILE.as_path(),
        PACKAGES_RPMBUILD_SRC_FILE.as_path()
    )
        .map_or_else(
            |error| Err(PackageError {
                message: format!(
                    "failed to copy file \"{}\" to \"{}\"",
                    PACKAGES_SRC_FILE.to_string_lossy(),
                    PACKAGES_RPMBUILD_SRC_FILE.to_string_lossy(),
                ),
                source: Some(error),
            }),
            |_| Ok(()),
        )?;

    // Populate spec file.
    let destination = File::create(PACKAGES_RPMBUILD_SPEC_FILE.as_path())
        .map_err(|error| PackageError {
            message: format!(
                "failed to create rpm spec file \"{}\"",
                PACKAGES_RPMBUILD_SPEC_FILE.to_string_lossy(),
            ),
            source: Some(error),
        })?;

    let mut files = String::new();
    INSTALL_FILES
        .iter()
        .map(|source_destination| source_destination
            .destination
            .to_string_lossy()
            .replacen(
                INSTALL_ROOT_DIR
                    .to_string_lossy()
                    .as_ref(),
                "%{app_destination_dir}",
                1,
            )
        )
        .for_each(|destination| files.push_str(&format!(
            "\\\n{}",
            destination,
        )));
    let mut sed_command = Command::new("sed");
    sed_command
        .args([
            format!(
                "--expression=s#^Name:$#Name: {}#",
                CARGO_NAME,
            ),
            format!(
                "--expression=s#^Version:$#Version: {}#",
                CARGO_VERSION,
            ),
            format!(
                "--expression=s#^Source0:$#Source0: {}#",
                PACKAGES_RPMBUILD_SRC_FILE.to_string_lossy(),
            ),
            format!(
                "--expression=\\#^%files$#a{}",
                files,
            ),
            ASSETS_RPM_SPEC_FILE.to_string_lossy().to_string(),
        ])
        .stdout(Stdio::from(destination));
    let mut sed_string = sed_command
        .get_program()
        .to_os_string();
    sed_command
        .get_args()
        .for_each(|arg| sed_string.push(format!(
            " {}",
            arg.to_string_lossy(),
        )));

    let sed_output = sed_command
        .output()
        .map_err(|error| PackageError {
            message: format!(
                "failed to execute sed command \"{}\"",
                sed_string.to_string_lossy(),
            ),
            source: Some(error),
        })?;
    if !sed_output.status.success() {
        eprintln!("==== command ====");
        eprintln!("{}", sed_string.to_string_lossy());

        // exit code
        let sed_exit_code = sed_output
            .status
            .code();
        eprintln!("==== exit code ====");
        eprintln!(
            "{}",
            sed_exit_code.map_or_else(
                || "<failed to retrieve>".to_string(),
                |status| status.to_string()
            )
        );

        // stdout
        let sed_stdout = from_utf8(&sed_output.stdout);
        eprintln!("==== stdout ====");
        if sed_stdout.is_ok() {
            eprintln!("{}", sed_stdout.unwrap_or("<failed to retrieve>"));
        }

        // stderr
        let sed_stderr = from_utf8(&sed_output.stderr);
        eprintln!("==== stderr ====");
        if sed_stderr.is_ok() {
            eprintln!("{}", sed_stderr.unwrap_or("<failed to retrieve>"));
        }

        return Err(PackageError {
            message: format!(
                "sed command \"{}\" failed",
                sed_string.to_string_lossy(),
            ),
            source: None,
        });
    }

    let mut rpmbuild_command = Command::new("rpmbuild");
    rpmbuild_command.args([
        format!(
            "--define=_topdir {}",
            PACKAGES_RPMBUILD_DIR.to_string_lossy()
        ),
        "-ba".to_string(),
        PACKAGES_RPMBUILD_SPEC_FILE.to_string_lossy().to_string()
    ]);
    let mut rpmbuild_string = rpmbuild_command
        .get_program()
        .to_os_string();
    rpmbuild_command
        .get_args()
        .for_each(|arg| rpmbuild_string.push(format!(
            " {}",
            arg.to_string_lossy(),
        )));

    let rpmbuild_output = rpmbuild_command
        .output()
        .map_err(|error| PackageError {
            message: format!(
                "failed to execute rpmbuild command \"{}\"",
                rpmbuild_string.to_string_lossy(),
            ),
            source: Some(error),
        })?;
    if !rpmbuild_output.status.success() {
        eprintln!("==== command ====");
        eprintln!("{}", rpmbuild_string.to_string_lossy());

        // exit code
        let rpmbuild_exit_code = rpmbuild_output
            .status
            .code();
        eprintln!("==== exit code ====");
        eprintln!(
            "{}",
            rpmbuild_exit_code.map_or_else(
                || "<failed to retrieve>".to_string(),
                |status| status.to_string()
            )
        );

        // stdout
        let rpmbuild_stdout = from_utf8(&rpmbuild_output.stdout);
        eprintln!("==== stdout ====");
        if rpmbuild_stdout.is_ok() {
            eprintln!("{}", rpmbuild_stdout.unwrap_or("<failed to retrieve>"));
        }

        // stderr
        let rpmbuild_stderr = from_utf8(&rpmbuild_output.stderr);
        eprintln!("==== stderr ====");
        if rpmbuild_stderr.is_ok() {
            eprintln!("{}", rpmbuild_stderr.unwrap_or("<failed to retrieve>"));
        }

        return Err(PackageError {
            message: format!(
                "rpmbuild command \"{}\" failed",
                rpmbuild_string.to_string_lossy(),
            ),
            source: None,
        });
    }

    Ok(())
}
