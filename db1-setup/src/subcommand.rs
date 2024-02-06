mod hostname;
mod incus;
mod install;
pub(crate) mod neovim;
pub(crate) mod ssh;
#[cfg(not(target_os = "windows"))]
pub(crate) mod tmux;


use crate::{
    error::Error,
    platform::Platform,
};
use hostname::subcommand_hostname;
use incus::subcommand_incus;
use install::subcommand_install;
use neovim::subcommand_neovim;
use ssh::subcommand_ssh;
#[cfg(not(target_os = "windows"))]
use tmux::subcommand_tmux;


#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    Hostname,
    Incus,
    Install,
    Neovim(neovim::Args),
    Ssh(ssh::Args),
    #[cfg(not(target_os = "windows"))]
    Tmux(tmux::Args),
}


pub(crate) fn execute_subcommand(
    platform: &Platform,
    args_subcommand: &Subcommand,
) -> Result<(), Error> {
    match args_subcommand {
        Subcommand::Hostname => subcommand_hostname(platform),
        Subcommand::Incus => subcommand_incus(platform),
        Subcommand::Install => subcommand_install(platform),
        Subcommand::Neovim(args) => subcommand_neovim(platform, &args),
        Subcommand::Ssh(args) => subcommand_ssh(platform, &args),
        #[cfg(not(target_os = "windows"))]
        Subcommand::Tmux(args) => subcommand_tmux(platform, &args),
    }
}
