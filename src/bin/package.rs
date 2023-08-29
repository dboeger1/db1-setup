use clap::Parser;
use dboeger1_dotfiles::*;
use std::{fs::{create_dir_all, remove_dir_all, copy, File}, process::{Command, Stdio}, io::Write};


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
struct Args {
    #[arg(short, long)]
    clean: bool,
}


fn main() -> Result<(), u8> {
    let args = Args::parse();

    match args.clean {
        true => {
            println!("Cleaning...");
            match remove_dir_all(PROJECT_PACKAGES_DIR.as_path()).is_ok() {
                true => Ok(()),
                false => Err(1),
            }
        },
        false => {
            println!("Building...");
            if PROJECT_PACKAGES_DIR.exists() {
                return Err(1);
            }

            // src
            if create_dir_all(PACKAGES_SRC_DIR.as_path()).is_err() {
                return Err(2);
            }
            let _ = Command::new("tar")
                .args([
                    "--create",
                    "--gzip",
                    format!(
                        "--directory={}",
                        &PROJECT_ASSETS_DIR.to_string_lossy()
                    ).as_str(),
                    format!(
                        "--file={}",
                        &PACKAGES_SRC_FILE.to_string_lossy()
                    ).as_str(),
                    format!(
                        "--transform=s#^#{}/#",
                        NAME_VERSION.as_str()
                    ).as_str(),
                    "neovim",
                    "tmux"
                ])
                .output()
                .unwrap();

            // deb
            if create_dir_all(PACKAGES_DPKG_DIR.as_path()).is_err() {
                return Err(3);
            }
            if copy(
                PACKAGES_SRC_FILE.as_path(),
                PACKAGES_DPKG_SRC_FILE.as_path()
            ).is_err() {
                return Err(4);
            }
            if copy(
                ASSETS_DEB_CONTROL_FILE.as_path(),
                PACKAGES_DPKG_CONTROL_FILE.as_path()
            ).is_err() {
                return Err(5);
            }

            // rpm
            for dir in [
                PACKAGES_RPMBUILD_BUILD_DIR.as_path(),
                PACKAGES_RPMBUILD_BUILDROOT_DIR.as_path(),
                PACKAGES_RPMBUILD_RPMS_DIR.as_path(),
                PACKAGES_RPMBUILD_SOURCES_DIR.as_path(),
                PACKAGES_RPMBUILD_SPECS_DIR.as_path(),
                PACKAGES_RPMBUILD_SRPMS_DIR.as_path()
            ] {
                if create_dir_all(dir).is_err() {
                    return Err(6);
                }
            }
            if copy(
                PACKAGES_SRC_FILE.as_path(),
                PACKAGES_RPMBUILD_SRC_FILE.as_path()
            ).is_err() {
                return Err(7);
            }

            let mut file = File::create(
                PACKAGES_RPMBUILD_SPEC_FILE.as_path()
            ).unwrap();
            let output = Command::new("sed")
                .args([
                    "-e", format!(
                        "s#^Name:$#Name: {}#",
                        CARGO_NAME
                    ).as_str(),
                    "-e", format!(
                        "s#^Version:$#Version: {}#",
                        CARGO_VERSION
                    ).as_str(),
                    "-e", format!(
                        "s#^Source0:$#Source0: {}#",
                        PACKAGES_RPMBUILD_SRC_FILE.to_string_lossy()
                    ).as_str(),
                    &ASSETS_RPM_SPEC_FILE.to_string_lossy(),
                    //">", &PACKAGES_RPMBUILD_SPEC_FILE.to_string_lossy()
                ])
                .stdout(Stdio::from(file.try_clone().unwrap()))
                .output()
                .unwrap();
            file.write_all(&output.stdout).unwrap();

            let stuff = Command::new("rpmbuild")
                .args([
                    format!(
                        "--define=_topdir {}",
                        PACKAGES_RPMBUILD_DIR.to_string_lossy()
                    ),
                    "-ba".to_string(),
                    PACKAGES_RPMBUILD_SPEC_FILE.to_string_lossy().to_string()
                ])
                .output()
                .unwrap();
                //.get_args()
                //.for_each(|arg| println!("{}", arg.to_string_lossy()));
            println!("{}", std::str::from_utf8(&stuff.stdout).unwrap());
            println!();
            println!("========");
            println!();
            println!("{}", std::str::from_utf8(&stuff.stderr).unwrap());

            Ok(())
        },
    }
}
