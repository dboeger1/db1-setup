use crate::{
    CARGO_NAME,
    FILES_CSV_NAME,
    NAME_VERSION,
    PROJECT_ASSETS_PLATFORM_DIR,
    PROJECT_PACKAGES_DIR,
};
use std::path::PathBuf;


pub const PLATFORM_NAME: &str = "linux";

lazy_static! {
    // Asset paths.
    pub static ref ASSETS_DIR: PathBuf =
        PROJECT_ASSETS_PLATFORM_DIR.join(PLATFORM_NAME);

    pub static ref ASSETS_FILES_CSV_FILE: PathBuf =
        ASSETS_DIR.join(FILES_CSV_NAME);

    pub static ref ASSETS_DEB_DIR: PathBuf =
        ASSETS_DIR.join("deb");
    pub static ref ASSETS_DEB_CONTROL_FILE: PathBuf =
        ASSETS_DEB_DIR.join("control");

    pub static ref ASSETS_RPM_DIR: PathBuf =
        ASSETS_DIR.join("rpm");
    pub static ref ASSETS_RPM_SPEC_FILE: PathBuf =
        ASSETS_RPM_DIR.join("name.spec");

    // Package paths.
    pub static ref PACKAGES_DIR: PathBuf =
        PROJECT_PACKAGES_DIR.join(PLATFORM_NAME);

    pub static ref PACKAGES_SRC_DIR: PathBuf =
        PACKAGES_DIR.join("src");
    pub static ref PACKAGES_SRC_NAME: String =
        format!("{}.tar.gz", NAME_VERSION.as_str());
    pub static ref PACKAGES_SRC_FILE: PathBuf =
        PACKAGES_SRC_DIR.join(PACKAGES_SRC_NAME.as_str());

    pub static ref PACKAGES_DEB_DIR: PathBuf =
        PACKAGES_DIR.join("deb");
    pub static ref PACKAGES_DPKG_DIR: PathBuf =
        PACKAGES_DEB_DIR.join("DEB");
    pub static ref PACKAGES_DPKG_CONTROL_FILE: PathBuf =
        PACKAGES_DPKG_DIR.join("control");
    pub static ref PACKAGES_DPKG_SRC_FILE: PathBuf =
        PACKAGES_DPKG_DIR.join(PACKAGES_SRC_NAME.as_str());

    pub static ref PACKAGES_RPM_DIR: PathBuf =
        PACKAGES_DIR.join("rpm");
    pub static ref PACKAGES_RPMBUILD_DIR: PathBuf =
        PACKAGES_RPM_DIR.join("rpmbuild");
    pub static ref PACKAGES_RPMBUILD_BUILD_DIR: PathBuf =
        PACKAGES_RPMBUILD_DIR.join("BUILD");
    pub static ref PACKAGES_RPMBUILD_BUILDROOT_DIR: PathBuf =
        PACKAGES_RPMBUILD_DIR.join("BUILDROOT");
    pub static ref PACKAGES_RPMBUILD_RPMS_DIR: PathBuf =
        PACKAGES_RPMBUILD_DIR.join("RPMS");
    pub static ref PACKAGES_RPMBUILD_SOURCES_DIR: PathBuf =
        PACKAGES_RPMBUILD_DIR.join("SOURCES");
    pub static ref PACKAGES_RPMBUILD_SRC_FILE: PathBuf =
        PACKAGES_RPMBUILD_SOURCES_DIR.join(PACKAGES_SRC_NAME.as_str());
    pub static ref PACKAGES_RPMBUILD_SPECS_DIR: PathBuf =
        PACKAGES_RPMBUILD_DIR.join("SPECS");
    pub static ref PACKAGES_RPM_SPEC_NAME: String =
        format!("{}.spec", CARGO_NAME);
    pub static ref PACKAGES_RPMBUILD_SPEC_FILE: PathBuf =
        PACKAGES_RPMBUILD_SPECS_DIR.join(PACKAGES_RPM_SPEC_NAME.as_str());
    pub static ref PACKAGES_RPMBUILD_SRPMS_DIR: PathBuf =
        PACKAGES_RPMBUILD_DIR.join("SRPMS");

    // Install paths.
    pub static ref INSTALL_ROOT_DIR: PathBuf =
        PathBuf::from("/opt/dboeger1-dotfiles");
    pub static ref INSTALL_FILES: Vec<(String, String)> =
        // read CSV file into string
        std::fs::read_to_string(ASSETS_FILES_CSV_FILE.as_path())
        .unwrap()
        .lines()
        // split comma-separated pairs
        .map(|line| {
            let (source, destination) = line.split_once(",").unwrap();
            (source.to_owned(), destination.to_owned())
        })
        .collect();
}
