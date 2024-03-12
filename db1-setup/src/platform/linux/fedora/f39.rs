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


use clap::Parser;
use crate::platform::Strategy;


pub(super) const STRATEGY: Strategy = || {
    let args = Args::parse();
    match args.subcommand {
        Subcommand::C(args) => c::execute(args),
        Subcommand::Cpp(args) => cpp::execute(args),
        Subcommand::Hostname(args) => hostname::execute(args),
        Subcommand::Incus(args) => incus::execute(args),
        Subcommand::Javascript(args) => javascript::execute(args),
        Subcommand::Neovim(args) => neovim::execute(args),
        Subcommand::Rpm(args) => rpm::execute(args),
        Subcommand::Rust(args) => rust::execute(args),
        Subcommand::Ssh(args) => ssh::execute(args),
        Subcommand::Tmux(args) => tmux::execute(args),
        Subcommand::Utilities(args) => utilities::execute(args),
    }
};


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    C(c::Args),
    Cpp(cpp::Args),
    Hostname(hostname::Args),
    Incus(incus::Args),
    Javascript(javascript::Args),
    Neovim(neovim::Args),
    Rpm(rpm::Args),
    Rust(rust::Args),
    Ssh(ssh::Args),
    Tmux(tmux::Args),
    Utilities(utilities::Args),
}
