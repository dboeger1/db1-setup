use clap::Parser;
use dboeger1_dotfiles::*;
use std::{
    error::Error,
    fmt::Display,
    fs::{
        File,
        copy,
        create_dir_all,
        remove_dir_all,
    },
    io,
    process::{
        Command,
        ExitCode,
        Stdio,
    },
    str::from_utf8,
};


#[derive(Debug)]
struct PackageError {
    message: String,
    source: Option<io::Error>,
}

impl Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PackageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|error| error as &(dyn Error + 'static))
    }
}


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
struct Args {
    #[arg(short, long)]
    clean: bool,
}

fn main() -> ExitCode {
    match Args::parse().clean {
        true => {
            match clean() {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("{}", error);
                    if let Some(source) = error.source() {
                        eprintln!("{}", source);
                    }
                    ExitCode::FAILURE
                }
            }
        },
        false => {
            match build() {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("{}", error);
                    if let Some(source) = error.source() {
                        eprintln!("source: {}", source);
                    }
                    ExitCode::FAILURE
                }
            }
        },
    }
}


fn clean() -> Result<(), PackageError> {
    match remove_dir_all(PROJECT_PACKAGES_DIR.as_path()) {
        Ok(_) => Ok(()),
        Err(error) => Err(PackageError {
            message: format!(
                "failed to remove packages directory \"{}\"",
                PROJECT_PACKAGES_DIR.to_string_lossy(),
            ),
            source: Some(error),
        }),
    }
}

fn build() -> Result<(), PackageError> {
    if PROJECT_PACKAGES_DIR.exists() {
        return Err(PackageError {
            message: format!(
                "cannot overwrite existing packages directory \"{}\"",
                PROJECT_PACKAGES_DIR.to_string_lossy(),
            ),
            source: None,
        });
    }

    build_src()?;
    build_deb()?;
    build_rpm()?;

    Ok(())
}


fn build_src() -> Result<(), PackageError> {
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
            &PROJECT_ASSETS_DIR.to_string_lossy(),
        ),
        format!(
            "--file={}",
            &PACKAGES_SRC_FILE.to_string_lossy(),
        ),
        format!(
            "--transform=s#^#{}/#",
            NAME_VERSION.as_str(),
        ),
        "neovim".to_string(),
        "tmux".to_string(),
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


fn build_deb() -> Result<(), PackageError> {
    create_dir_all(PACKAGES_DPKG_DIR.as_path())
        .map_err(|error| PackageError {
            message: format!(
                "failed to create deb directory \"{}\"",
                PACKAGES_DPKG_DIR.to_string_lossy(),
            ),
            source: Some(error),
        })?;

    for (source, destination) in [
        (
            PACKAGES_SRC_FILE.as_path(),
            PACKAGES_DPKG_SRC_FILE.as_path(),
        ),
        (
            ASSETS_DEB_CONTROL_FILE.as_path(),
            PACKAGES_DPKG_CONTROL_FILE.as_path(),
        ),
    ] {
        copy(source, destination)
            .map_or_else(
                |error| Err(PackageError {
                    message: format!(
                        "failed to copy file \"{}\" to \"{}\"",
                        source.to_string_lossy(),
                        destination.to_string_lossy(),
                    ),
                    source: Some(error),
                }),
                |_| Ok(()),
            )?;
    }

    Ok(())
}


fn build_rpm() -> Result<(), PackageError> {
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
        .map(|(_, destination)| destination
            .to_string()
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
