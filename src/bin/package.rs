use clap::Parser;
use dboeger1_dotfiles::*;
use std::fs::remove_dir_all;

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
            match PROJECT_PACKAGES_DIR.exists() {
                true => Err(1),
                false => Ok(()),
            }
        },
    }
}

//    # src
//    mkdir -p ${BUILD_SRC_DIR}
//    tar -c -C "${PROJECT_SRC_DIR}" -f "${src_file_path}" \
//        --transform="s#^#${src_name}/#" \
//        neovim tmux
//
//    # deb
//    mkdir -p ${DPKG_DIR}
//    cp ${src_file_path} ${dpkg_src_file_path}
//    # TODO: Replace following line with actual file modification and packaging.
//    cp ${DEB_CONTROL_FILE_PATH} ${DPKG_CONTROL_FILE_PATH}
//
//    # rpm
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
