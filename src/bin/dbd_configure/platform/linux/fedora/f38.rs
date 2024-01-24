use crate::{
    error::Error,
    platform::{
        INSTALL_DIR,
        linux::fedora::dnf_install,
        Platform,
    },
    source_destination::SourceDestination,
};
use dboeger1_dotfiles::HOME_DIR;
use lazy_static::lazy_static;


// Platform data.
pub(crate) const PLATFORM: PlatformFedora38 = PlatformFedora38 {};

pub(crate) struct PlatformFedora38 {}

impl Platform for PlatformFedora38 {
    fn get_neovim_paths(&self) -> Option<SourceDestination> {
        Some(SourceDestination {
            source: INSTALL_DIR.join("neovim"),
            destination: HOME_DIR.join(".config/nvim"),
        })
    }

    fn get_tmux_paths(&self) -> Option<SourceDestination> {
        Some(SourceDestination {
            source: INSTALL_DIR.join("tmux/.tmux.conf"),
            destination: HOME_DIR.join(".tmux.conf"),
        })
    }

    fn get_install_packages(&self) -> Option<fn() -> Result<(), Error>> {
       Some(|| dnf_install(&*PACKAGES))
    }
}


// Packages to install.
lazy_static! {
        static ref PACKAGES: Vec<&'static str> = PACKAGES_STRING
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                match line.is_empty() {
                    true => None,
                    false => Some(line),
                }
            })
            .collect();
}

const PACKAGES_STRING: &str = concat!(
    // Utilities
    r#"
    bash
    coreutils
    diffutils
    fd-find
    git
    patch
    ripgrep
    tree
    unzip
    wget
    "#,

    // Applications
    r#"
    neovim
    "#,

    // C
    r#"
    cmake
    gcc
    make
    "#,

    // C++
    r#"
    gcc-c++
    "#,

    // Rust
    r#"
    rustup
    "#,

    // Python
    r#"
    python
    "#,

    // Javascript
    r#"
    npm
    "#,

    // RPM
    r#"
    rpm-build
    rpm-devel
    rpmdevtools
    rpmlint
    "#,
);
