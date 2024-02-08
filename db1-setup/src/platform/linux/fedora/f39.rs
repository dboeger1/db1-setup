mod c;
mod cpp;
mod hostname;
mod incus;
mod javascript;
mod neovim;
mod rpm;
mod rust;
mod ssh;
mod tmux;
mod utilities;


use crate::{
    HOME_DIR,
    platform::{
        linux::INSTALL_DIR,
        Platform,
    },
};
use lazy_static::lazy_static;


lazy_static! {
    // Platform data.
    pub(crate) static ref PLATFORM: Platform = Platform {
        // C
        c_install: c::install,
        c_verify: c::verify,

        // C++
        cpp_install: cpp::install,
        cpp_verify: cpp::verify,

        // Hostname
        hostname_configure: hostname::configure,
        hostname_verify: hostname::verify,

        // Incus
        incus_install: incus::install,
        incus_verify: incus::verify,

        // Javascript
        javascript_install: javascript::install,
        javascript_verify: javascript::verify,

        // Neovim
        neovim_destination: Some(HOME_DIR.join(".config/nvim")),
        neovim_source: Some(INSTALL_DIR.join("neovim")),
        neovim_install: neovim::install,
        neovim_verify: neovim::verify,

        // RPM
        rpm_install: rpm::install,
        rpm_verify: rpm::verify,

        // Rust
        rust_install: rust::install,
        rust_verify: rust::verify,

        // SSH
        ssh_configure: ssh::configure,
        ssh_verify: ssh::verify,

        // tmux
        tmux_destination: Some(HOME_DIR.join(".tmux.conf")),
        tmux_source: Some(INSTALL_DIR.join("tmux/.tmux.conf")),
        tmux_install: tmux::install,
        tmux_verify: tmux::verify,

        // Utilities
        utilities_install: utilities::install,
        utilities_verify: utilities::verify,
    };
}
