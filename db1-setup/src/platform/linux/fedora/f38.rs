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


use crate::platform::Platform;
use lazy_static::lazy_static;


lazy_static! {
    pub(super) static ref PLATFORM: Platform = Platform {
        c: c::PLATFORM.clone(),
        cpp: cpp::PLATFORM.clone(),
        hostname: hostname::PLATFORM.clone(),
        incus: incus::PLATFORM.clone(),
        javascript: javascript::PLATFORM.clone(),
        neovim: neovim::PLATFORM.clone(),
        rpm: rpm::PLATFORM.clone(),
        rust: rust::PLATFORM.clone(),
        ssh: ssh::PLATFORM.clone(),
        tmux: tmux::PLATFORM.clone(),
        utilities: utilities::PLATFORM.clone(),
    };
}
