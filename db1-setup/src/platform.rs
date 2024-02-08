mod c;
mod cpp;
mod hostname;
#[cfg(target_os = "linux")]
mod incus;
mod javascript;
mod neovim;
#[cfg(target_os = "linux")]
mod rpm;
mod rust;
mod ssh;
#[cfg(not(target_os = "windows"))]
mod tmux;
mod utilities;

#[cfg(target_os = "linux")]
mod linux;


#[cfg(target_os = "linux")]
pub(crate) use linux::PLATFORM;


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
