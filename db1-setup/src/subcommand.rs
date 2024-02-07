mod c;
mod cpp;
mod hostname;
mod incus;
mod javascript;
pub(crate) mod neovim;
mod rust;
pub(crate) mod ssh;
mod utilities;

#[cfg(target_os = "linux")]
mod rpm;

#[cfg(not(target_os = "windows"))]
pub(crate) mod tmux;


use crate::{
    error::Error,
    platform::Platform,
};
use c::subcommand_c;
use cpp::subcommand_cpp;
use hostname::subcommand_hostname;
use incus::subcommand_incus;
use javascript::subcommand_javascript;
use neovim::subcommand_neovim;
use rust::subcommand_rust;
use ssh::subcommand_ssh;
use utilities::subcommand_utilities;

#[cfg(target_os = "linux")]
use rpm::subcommand_rpm;

#[cfg(not(target_os = "windows"))]
use tmux::subcommand_tmux;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    C(c::Args),
    Cpp(cpp::Args),
    Hostname(hostname::Args),
    Incus(incus::Args),
    Javascript(javascript::Args),
    Neovim(neovim::Args),
    Rust(rust::Args),
    Ssh(ssh::Args),
    Utilities(utilities::Args),

    #[cfg(target_os = "linux")]
    Rpm(rpm::Args),

    #[cfg(not(target_os = "windows"))]
    Tmux(tmux::Args),
}


pub(crate) fn execute_subcommand(
    platform: &Platform,
    args_subcommand: &Subcommand,
) -> Result<(), Error> {
    match args_subcommand {
        Subcommand::C(args) => subcommand_c(platform, &args),
        Subcommand::Cpp(args) => subcommand_cpp(platform, &args),
        Subcommand::Hostname(args) => subcommand_hostname(platform, &args),
        Subcommand::Incus(args) => subcommand_incus(platform, &args),
        Subcommand::Javascript(args) => subcommand_javascript(platform, &args),
        Subcommand::Neovim(args) => subcommand_neovim(platform, &args),
        Subcommand::Rust(args) => subcommand_rust(platform, &args),
        Subcommand::Ssh(args) => subcommand_ssh(platform, &args),
        Subcommand::Utilities(args) => subcommand_utilities(platform, &args),

        #[cfg(target_os = "linux")]
        Subcommand::Rpm(args) => subcommand_rpm(platform, &args),

        #[cfg(not(target_os = "windows"))]
        Subcommand::Tmux(args) => subcommand_tmux(platform, &args),
    }
}
