use clap::Parser;
use dboeger1_dotfiles::*;
use std::{fs::{create_dir_all, remove_dir_all, copy}, process::Command};

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
                        CARGO_NAME
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
            //    mkdir -p \
            //        ${RPMBUILD_BUILD_DIR}       \
            //        ${RPMBUILD_BUILDROOT_DIR}   \
            //        ${RPMBUILD_RPMS_DIR}        \
            //        ${RPMBUILD_SOURCES_DIR}     \
            //        ${RPMBUILD_SPECS_DIR}       \
            //        ${RPMBUILD_SRPMS_DIR}
            //    cp ${src_file_path} ${rpmbuild_src_file_path}
            //    sed \
            //        -e "s#^Name:\$#Name: ${PROJECT_NAME}#" \
            //        -e "s#^Version:\$#Version: ${arg_version}#" \
            //        -e "s#^Source0:\$#Source0: ${rpmbuild_src_file_path}#" \
            //        ${RPM_SPEC_FILE_PATH} > ${RPMBUILD_SPEC_FILE_PATH}
            //    rpmbuild --define "_topdir ${RPMBUILD_DIR}" -ba "${RPMBUILD_SPEC_FILE_PATH}"

            Ok(())
        },
    }
}
