use clap::Parser;
use dboeger1_dotfiles::*;
use std::{
    fs::{
        copy,
        create_dir_all,
        File,
        remove_dir_all,
    },
    //io::Write,
    process::{
        Command,
        Stdio,
    },
};


#[derive(Debug)]
enum ExitCode {
    First,
}


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
struct Args {
    #[arg(short, long)]
    clean: bool,
}

fn main() -> Result<(), ExitCode> {
    let args = Args::parse();

    match args.clean {
        true => clean()?,
        false => build()?,
    };

    Ok(())
}


fn clean() -> Result<(), ExitCode> {
    match remove_dir_all(PROJECT_PACKAGES_DIR.as_path()) {
        Ok(_) => {
            println!("Deleted packages directory.");
            Ok(())
        },
        Err(_) => {
            eprintln!("Failed to delete packages directory.");
            Err(ExitCode::First)
        },
    }
}

fn build() -> Result<(), ExitCode> {
    if PROJECT_PACKAGES_DIR.exists() {
        eprintln!("Cannot overwrite existing packages directory.");
        return Err(ExitCode::First);
    }

    build_src()?;
    build_deb()?;
    build_rpm()?;

    Ok(())
}


// src
fn init_src() -> Result<(), ExitCode> {
    create_dir_all(PACKAGES_SRC_DIR.as_path())
        .map_err(|_| return ExitCode::First)
}

fn build_src() -> Result<(), ExitCode> {
    init_src()?;

    let mut tar_command = Command::new("tar");
    tar_command.args([
        "--create".to_string(),
        "--gzip".to_string(),
        format!(
            "--directory={}",
            &PROJECT_ASSETS_DIR.to_string_lossy()
        ),
        format!(
            "--file={}",
            &PACKAGES_SRC_FILE.to_string_lossy()
        ),
        format!(
            "--transform=s#^#{}/#",
            NAME_VERSION.as_str()
        ),
        "neovim".to_string(),
        "tmux".to_string()
    ]);

    let tar_output = tar_command
        .output()
        .map_err(|_| ExitCode::First)?;
    if !tar_output.status.success() {
        eprintln!("tar command failed.");
        if tar_output.status.code().is_some() {
            eprintln!(
                "\n==== exit status: {} ====\n",
                tar_output.status.code().unwrap()
            );
        }
        eprintln!(
            "\n==== stdout ====\n{}",
            std::str::from_utf8(&tar_output.stdout).unwrap()
        );
        eprintln!(
            "\n==== stderr ====\n{}",
            std::str::from_utf8(&tar_output.stderr).unwrap()
        );

        return Err(ExitCode::First);
    }

    Ok(())
}


// deb
fn init_deb() -> Result<(), ExitCode> {
    create_dir_all(PACKAGES_DPKG_DIR.as_path())
        .map_err(|_| return ExitCode::First)?;

    for (source, destination) in [
        (
            PACKAGES_SRC_FILE.as_path(),
            PACKAGES_DPKG_SRC_FILE.as_path()
        ),
        (
            ASSETS_DEB_CONTROL_FILE.as_path(),
            PACKAGES_DPKG_CONTROL_FILE.as_path()
        ),
    ] {
        copy(source, destination)
            .map_or_else(
                |_| Err(ExitCode::First),
                |_| Ok(())
            )?;
    }

    Ok(())
}

fn build_deb() -> Result<(), ExitCode> {
    init_deb()?;

    Ok(())
}


// rpm
fn init_rpm() -> Result<(), ExitCode> {
    for dir in [
        PACKAGES_RPMBUILD_BUILD_DIR.as_path(),
        PACKAGES_RPMBUILD_BUILDROOT_DIR.as_path(),
        PACKAGES_RPMBUILD_RPMS_DIR.as_path(),
        PACKAGES_RPMBUILD_SOURCES_DIR.as_path(),
        PACKAGES_RPMBUILD_SPECS_DIR.as_path(),
        PACKAGES_RPMBUILD_SRPMS_DIR.as_path()
    ] {
        create_dir_all(dir)
            .map_err(|_| ExitCode::First)?;
    }

    copy(
        PACKAGES_SRC_FILE.as_path(),
        PACKAGES_RPMBUILD_SRC_FILE.as_path()
    )
        .map_or_else(
            |_| Err(ExitCode::First),
            |_| Ok(())
        )?;

    // Populate spec file.
    let destination = File::create(PACKAGES_RPMBUILD_SPEC_FILE.as_path())
        .map_err(|_| ExitCode::First)?;

    let mut sed_command = Command::new("sed");
    sed_command
        .args([
            format!(
                "--expression=s#^Name:$#Name: {}#",
                CARGO_NAME
            ),
            format!(
                "--expression=s#^Version:$#Version: {}#",
                CARGO_VERSION
            ),
            format!(
                "--expression=s#^Source0:$#Source0: {}#",
                PACKAGES_RPMBUILD_SRC_FILE.to_string_lossy()
            ),
            ASSETS_RPM_SPEC_FILE.to_string_lossy().to_string(),
        ])
        .stdout(Stdio::from(destination));

    let sed_output = sed_command
        .output()
        .map_err(|_| ExitCode::First)?;
    if !sed_output.status.success() {
        eprintln!("sed command failed.");
        if sed_output.status.code().is_some() {
            eprintln!(
                "\n==== exit status: {} ====\n",
                sed_output.status.code().unwrap()
            );
        }
        eprintln!(
            "\n==== stdout ====\n{}",
            std::str::from_utf8(&sed_output.stdout).unwrap()
        );
        eprintln!(
            "\n==== stderr ====\n{}",
            std::str::from_utf8(&sed_output.stderr).unwrap()
        );

        return Err(ExitCode::First);
    }

    //destination.write_all(&sed_output.stdout).unwrap();

    Ok(())
}

fn build_rpm() -> Result<(), ExitCode> {
    init_rpm()?;

    let mut rpmbuild_command = Command::new("rpmbuild");
    rpmbuild_command.args([
        format!(
            "--define=_topdir {}",
            PACKAGES_RPMBUILD_DIR.to_string_lossy()
        ),
        "-ba".to_string(),
        PACKAGES_RPMBUILD_SPEC_FILE.to_string_lossy().to_string()
    ]);

    let rpmbuild_output = rpmbuild_command
        .output()
        .map_err(|_| ExitCode::First)?;
    if !rpmbuild_output.status.success() {
        eprintln!("rpmbuild command failed.");
        if rpmbuild_output.status.code().is_some() {
            eprintln!(
                "\n==== exit status: {} ====\n",
                rpmbuild_output.status.code().unwrap()
            );
        }
        eprintln!(
            "\n==== stdout ====\n{}",
            std::str::from_utf8(&rpmbuild_output.stdout).unwrap()
        );
        eprintln!(
            "\n==== stderr ====\n{}",
            std::str::from_utf8(&rpmbuild_output.stderr).unwrap()
        );

        return Err(ExitCode::First);
    }

    Ok(())
}
