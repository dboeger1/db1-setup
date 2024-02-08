mod error;

pub mod c;
pub mod cpp;
pub mod hostname;
#[cfg(target_os = "linux")]
pub mod incus;
pub mod javascript;
pub mod neovim;
#[cfg(target_os = "linux")]
pub mod rpm;
pub mod rust;
pub mod ssh;
#[cfg(not(target_os = "windows"))]
pub mod tmux;
pub mod utilities;

mod platform;


use clap::Parser;
use error::Error;
use lazy_static::lazy_static;
use os_info::{
    get,
    Info,
};
use platform::{
    Platform,
    PLATFORM,
};
use std::{
    env::var,
    path::PathBuf,
};


pub const CARGO_NAME: &str = env!("CARGO_PKG_NAME");

lazy_static! {
    pub static ref OS_INFO: Info = get();
    pub(crate) static ref HOME_DIR: PathBuf =
        PathBuf::from(var("HOME").unwrap());
}


#[derive(clap::Subcommand, PartialEq, Eq)]
pub enum Subcommand {
    C(c::Args),
    Cpp(cpp::Args),
    Hostname(hostname::Args),
    #[cfg(target_os = "linux")]
    Incus(incus::Args),
    Javascript(javascript::Args),
    Neovim(neovim::Args),
    #[cfg(target_os = "linux")]
    Rpm(rpm::Args),
    Rust(rust::Args),
    Ssh(ssh::Args),
    #[cfg(not(target_os = "windows"))]
    Tmux(tmux::Args),
    Utilities(utilities::Args),
}

#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}


pub fn execute(args: &Args) -> Result<(), Error> {
    // Get platform strategy.
    let platform: &Platform;
    match PLATFORM.as_ref() {
        Some(platform_inner) => platform = platform_inner,
        None => return Err(Error {
            message: "Unrecognized platform. Aborting.".to_string(),
            source: None,
        }),
    }

    // Execute subcommand.
    match &args.subcommand {
        Subcommand::C(args) => c::execute(
            &platform.c,
            &args,
        ),
        Subcommand::Cpp(args) => cpp::execute(
            &platform.cpp,
            &args,
        ),
        Subcommand::Hostname(args) => hostname::execute(
            &platform.hostname,
            &args,
        ),
        #[cfg(target_os = "linux")]
        Subcommand::Incus(args) => incus::execute(
            &platform.incus,
            &args,
        ),
        Subcommand::Javascript(args) => javascript::execute(
            &platform.javascript,
            &args,
        ),
        Subcommand::Neovim(args) => neovim::execute(
            &platform.neovim,
            &args,
        ),
        #[cfg(target_os = "linux")]
        Subcommand::Rpm(args) => rpm::execute(
            &platform.rpm,
            &args,
        ),
        Subcommand::Rust(args) => rust::execute(
            &platform.rust,
            &args,
        ),
        Subcommand::Ssh(args) => ssh::execute(
            &platform.ssh,
            &args,
        ),
        #[cfg(not(target_os = "windows"))]
        Subcommand::Tmux(args) => tmux::execute(
            &platform.tmux,
            &args,
        ),
        Subcommand::Utilities(args) => utilities::execute(
            &platform.utilities,
            &args,
        ),
    }
}
