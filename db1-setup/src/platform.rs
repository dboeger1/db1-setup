pub(crate) mod c;
pub(crate) mod cpp;
pub(crate) mod hostname;
#[cfg(target_os = "linux")]
pub(crate) mod incus;
pub(crate) mod javascript;
pub(crate) mod neovim;
#[cfg(target_os = "linux")]
pub(crate) mod rpm;
pub(crate) mod rust;
pub(crate) mod ssh;
#[cfg(not(target_os = "windows"))]
pub(crate) mod tmux;
pub(crate) mod utilities;

#[cfg(target_os = "linux")]
mod linux;


use lazy_static::lazy_static;


lazy_static! {
    #[cfg(target_os = "linux")]
    pub(super) static ref PLATFORM: Option<Platform> = linux::PLATFORM.clone();
}


#[derive(Clone)]
pub(crate) struct Platform {
    pub(crate) c: c::Platform,
    pub(crate) cpp: cpp::Platform,
    pub(crate) hostname: hostname::Platform,
    #[cfg(target_os = "linux")]
    pub(crate) incus: incus::Platform,
    pub(crate) javascript: javascript::Platform,
    pub(crate) neovim: neovim::Platform,
    #[cfg(target_os = "linux")]
    pub(crate) rpm: rpm::Platform,
    pub(crate) rust: rust::Platform,
    pub(crate) ssh: ssh::Platform,
    #[cfg(not(target_os = "windows"))]
    pub(crate) tmux: tmux::Platform,
    pub(crate) utilities: utilities::Platform,
}
