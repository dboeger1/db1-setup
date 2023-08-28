#[macro_use]
extern crate lazy_static;

use std::{
    env::current_exe,
    path::PathBuf,
};

// Basic information.
pub const CARGO_NAME: &str = env!("CARGO_PKG_NAME");
pub const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");

// Asset paths.
pub const TARGET_DARWIN_NAME: &str = "darwin";
pub const TARGET_LINUX_NAME: &str = "linux";
pub const TARGET_WINDOWS_NAME: &str = "windows";

pub const FILES_CSV_NAME: &str = "files.csv";

// Static references computed at runtime.
lazy_static! {
    // Basic information.
    pub static ref NAME_VERSION: String = format!("{}-{}",
        CARGO_NAME,
        CARGO_VERSION
    );

    // Project root path.
    pub static ref PROJECT_ROOT_DIR: PathBuf =
        current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .ancestors()
        .find(|path| {
            path
                .read_dir()
                .unwrap()
                .map(|entry| entry.unwrap())
                .find(|entry| entry.path().ends_with("Cargo.toml"))
                .is_some()
        })
        .unwrap()
        .to_path_buf();

    // Asset paths.
    pub static ref ASSETS_DIR: PathBuf = PROJECT_ROOT_DIR.join("assets");

    pub static ref ASSETS_DARWIN_DIR: PathBuf =
        ASSETS_DIR.join(TARGET_DARWIN_NAME);
    pub static ref ASSETS_FILES_CSV_DARWIN_FILE: PathBuf =
        ASSETS_DARWIN_DIR.join(FILES_CSV_NAME);

    pub static ref ASSETS_LINUX_DIR: PathBuf =
        ASSETS_DIR.join(TARGET_LINUX_NAME);
    pub static ref ASSETS_FILES_CSV_LINUX_FILE: PathBuf =
        ASSETS_LINUX_DIR.join(FILES_CSV_NAME);

    pub static ref ASSETS_WINDOWS_DIR: PathBuf =
        ASSETS_DIR.join(TARGET_WINDOWS_NAME);
    pub static ref ASSETS_FILES_CSV_WINDOWS_FILE: PathBuf =
        ASSETS_WINDOWS_DIR.join(FILES_CSV_NAME);

    pub static ref ASSETS_NEOVIM_DIR: PathBuf = ASSETS_DIR.join("neovim");

    pub static ref ASSETS_TMUX_DIR: PathBuf = ASSETS_DIR.join("tmux");

    pub static ref ASSETS_DEB_DIR: PathBuf = ASSETS_DIR.join("deb");
    pub static ref ASSETS_DEB_CONTROL_FILE: PathBuf =
        ASSETS_DEB_DIR.join("control");

    pub static ref ASSETS_RPM_DIR: PathBuf = ASSETS_DIR.join("rpm");
    pub static ref ASSETS_RPM_SPEC_FILE: PathBuf =
        ASSETS_RPM_DIR.join("name.spec");

    // Package paths.
    pub static ref PACKAGES_DIR: PathBuf = PROJECT_ROOT_DIR.join("packages");

    pub static ref PACKAGES_DARWIN_DIR: PathBuf =
        PACKAGES_DIR.join(TARGET_DARWIN_NAME);

    pub static ref PACKAGES_LINUX_DIR: PathBuf =
        PACKAGES_DIR.join(TARGET_LINUX_NAME);
    pub static ref PACKAGES_LINUX_SRC_DIR: PathBuf =
        PACKAGES_LINUX_DIR.join("src");
    pub static ref PACKAGES_LINUX_DEB_DIR: PathBuf =
        PACKAGES_LINUX_DIR.join("deb");
    pub static ref PACKAGES_LINUX_DEB_DPKG_DIR: PathBuf =
        PACKAGES_LINUX_DEB_DIR.join("DEB");
    pub static ref PACKAGES_LINUX_DEB_CONTROL_FILE: PathBuf =
        PACKAGES_LINUX_DEB_DPKG_DIR.join("control");
    pub static ref PACKAGES_LINUX_RPM_DIR: PathBuf =
        PACKAGES_LINUX_DIR.join("rpm");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_DIR: PathBuf =
        PACKAGES_LINUX_RPM_DIR.join("rpmbuild");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_BUILD_DIR: PathBuf =
        PACKAGES_LINUX_RPM_RPMBUILD_DIR.join("BUILD");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_BUILDROOT_DIR: PathBuf =
        PACKAGES_LINUX_RPM_RPMBUILD_DIR.join("BUILDROOT");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_RPMS_DIR: PathBuf =
        PACKAGES_LINUX_RPM_RPMBUILD_DIR.join("RPMS");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_SOURCES_DIR: PathBuf =
        PACKAGES_LINUX_RPM_RPMBUILD_DIR.join("SOURCES");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_SPECS_DIR: PathBuf =
        PACKAGES_LINUX_RPM_RPMBUILD_DIR.join("SPECS");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_SRPMS_DIR: PathBuf =
        PACKAGES_LINUX_RPM_RPMBUILD_DIR.join("SRPMS");
    pub static ref PACKAGES_LINUX_RPM_RPMBUILD_SPEC_FILE: PathBuf =
        PACKAGES_LINUX_RPM_RPMBUILD_SPECS_DIR.join("name.spec");

    pub static ref PACKAGES_WINDOWS_DIR: PathBuf =
        PACKAGES_DIR.join(TARGET_WINDOWS_NAME);

    // Install paths.
    pub static ref FILES_LINUX: Vec<(String, String)> =
        std::fs::read_to_string(ASSETS_FILES_CSV_LINUX_FILE.as_path())
        .unwrap()
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .collect();
}
