use crate::{
    error::Error, platform::Platform, values::{
        DIR_PROJECT_GIT,
        DIR_PROJECT_PACKAGES,
        DIR_PROJECT_ROOT,
        DIR_PROJECT_TARGET,
        FILE_TAR,
    }
};
use lazy_static::lazy_static;
use std::{process::Command, str::from_utf8};


lazy_static! {
    pub(crate) static ref PLATFORM: Platform = Platform {
        archive_sources: || {
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
                    DIR_PROJECT_GIT.to_string_lossy(),
                ),
                format!(
                    "--exclude={}",
                    DIR_PROJECT_TARGET.to_string_lossy(),
                ),
                format!(
                    "--exclude={}",
                    DIR_PROJECT_PACKAGES.to_string_lossy(),
                ),
                format!(
                    "--file={}",
                    FILE_TAR.to_string_lossy(),
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
                .map_err(|error| Error {
                    message: format!(
                        "failed to execute command: \"{}\"",
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
                        |status| status.to_string(),
                    ),
                );

                // stdout
                let tar_stdout = from_utf8(&tar_output.stdout);
                eprintln!("==== stdout ====");
                if tar_stdout.is_ok() {
                    eprintln!(
                        "{}",
                        tar_stdout.unwrap_or("<failed to retrieve>"),
                    );
                }

                // stderr
                let tar_stderr = from_utf8(&tar_output.stderr);
                eprintln!("==== stderr ====");
                if tar_stderr.is_ok() {
                    eprintln!(
                        "{}",
                        tar_stderr.unwrap_or("<failed to retrieve>"),
                    );
                }

                return Err(Error {
                    message: format!(
                        "tar command failed: \"{}\"",
                        tar_string.to_string_lossy(),
                    ),
                    source: None,
                });
            }

            Ok(())
        },
    };
}
