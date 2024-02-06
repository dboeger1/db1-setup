use crate::{
    HOME_DIR,
    platform::{
        linux::{
            dnf::{
                copr_enable,
                install,
            },
            INSTALL_DIR,
            rust::rustup_init,
            ssh::configure_ssh,
        },
        Platform,
    },
    source_destination::SourceDestination,
};
use lazy_static::lazy_static;


lazy_static! {
    // Platform data.
    pub(crate) static ref PLATFORM: Platform = Platform {
        configure_ssh: configure_ssh,
        install_packages: || {
            install(["dnf-command(copr)"])?;
            copr_enable("ganto/lxc4")?;
            install(&*PACKAGES)?;
            rustup_init()?;

            Ok(())
        },
        neovim_paths: SourceDestination {
                source: INSTALL_DIR.join("neovim"),
                destination: HOME_DIR.join(".config/nvim"),
            },
        tmux_paths: SourceDestination {
                source: INSTALL_DIR.join("tmux/.tmux.conf"),
                destination: HOME_DIR.join(".tmux.conf"),
            },
    };

    // Packages to install.
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
    fd-find
    git
    incus
    patch
    ripgrep
    tree
    wget
    zip
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

    // Javascript
    r#"
    nodejs-npm
    "#,

    // RPM
    r#"
    rpm-build
    rpm-devel
    rpmdevtools
    rpmlint
    "#,
);
