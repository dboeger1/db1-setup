use crate::package_error::PackageError;
use dboeger1_dotfiles::*;
use std::{
    fs::create_dir_all,
    process::Command,
    str::from_utf8,
};


pub(crate) fn build_src() -> Result<(), PackageError> {
    create_dir_all(PACKAGES_SRC_DIR.as_path())
        .map_err(|error| PackageError {
            message: format!(
                "failed to create src directory \"{}\"",
                PACKAGES_SRC_DIR.to_string_lossy(),
            ),
            source: Some(error),
        })?;

    let mut tar_command = Command::new("tar");
    tar_command.args([
        "--create".to_string(),
        "--gzip".to_string(),
        format!(
            "--directory={}",
            PROJECT_ROOT_DIR.to_string_lossy(),
        ),
        format!(
            "--exclude={}",
            PROJECT_GIT_DIR
                .file_name()
                .unwrap()
                .to_string_lossy(),
        ),
        format!(
            "--exclude={}",
            PROJECT_TARGET_DIR
                .file_name()
                .unwrap()
                .to_string_lossy(),
        ),
        format!(
            "--exclude={}",
            PROJECT_PACKAGES_DIR
                .file_name()
                .unwrap()
                .to_string_lossy(),
        ),
        format!(
            "--file={}",
            PACKAGES_SRC_FILE.to_string_lossy(),
        ),
        format!(
            "--transform=s#^.#{}#",
            NAME_VERSION.as_str(),
        ),
        ".".to_string(),
    ]);
    let mut tar_string = tar_command
        .get_program()
        .to_os_string();
    tar_command
        .get_args()
        .for_each(|arg| tar_string.push(format!(
            " {}",
            arg.to_string_lossy(),
        )));

    let tar_output = tar_command
        .output()
        .map_err(|error| PackageError {
            message: format!(
                "failed to execute tar command \"{}\"",
                tar_string.to_string_lossy(),
            ),
            source: Some(error),
        })?;
    if !tar_output.status.success() {
        // command
        eprintln!("==== command ====");
        eprintln!("{}", tar_string.to_string_lossy());

        // exit code
        let tar_exit_code = tar_output
            .status
            .code();
        eprintln!("==== exit code ====");
        eprintln!(
            "{}",
            tar_exit_code.map_or_else(
                || "<failed to retrieve>".to_string(),
                |status| status.to_string()
            )
        );

        // stdout
        let tar_stdout = from_utf8(&tar_output.stdout);
        eprintln!("==== stdout ====");
        if tar_stdout.is_ok() {
            eprintln!("{}", tar_stdout.unwrap_or("<failed to retrieve>"));
        }

        // stderr
        let tar_stderr = from_utf8(&tar_output.stderr);
        eprintln!("==== stderr ====");
        if tar_stderr.is_ok() {
            eprintln!("{}", tar_stderr.unwrap_or("<failed to retrieve>"));
        }

        return Err(PackageError {
            message: format!(
                "tar command \"{}\" failed",
                tar_string.to_string_lossy(),
            ),
            source: None,
        });
    }

    Ok(())
}
