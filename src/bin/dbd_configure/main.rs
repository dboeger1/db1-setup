mod args;
mod error;
mod git;
mod incus;
mod install;
mod neovim;
mod platform;
mod source_destination;
mod ssh;
mod tmux;


use clap::Parser;
use crate::{
    args::{
        Args,
        ArgsSubcommand,
    },
    git::subcommand_git,
    incus::subcommand_incus,
    install::subcommand_install,
    neovim::subcommand_neovim,
    ssh::subcommand_ssh,
    tmux::subcommand_tmux,
};
use platform::{
    Platform,
    PLATFORM,
};
use std::{
    error::Error,
    process::ExitCode,
};


fn main() -> ExitCode {
    // Get platform data.
    let platform: &Platform;
    match PLATFORM.as_ref() {
        Some(platform_data) => platform = platform_data,
        None => {
            eprintln!("Unrecognized platform. Aborting.");
            return ExitCode::FAILURE;
        },
    }

    // Parse arguments.
    let args = Args::parse();

    // Execute subcommand.
    if let Err(error) = match args.subcommand {
        ArgsSubcommand::Git => subcommand_git(platform),
        ArgsSubcommand::Incus => subcommand_incus(platform),
        ArgsSubcommand::Install => subcommand_install(platform),
        ArgsSubcommand::Neovim => subcommand_neovim(platform),
        ArgsSubcommand::Ssh => subcommand_ssh(platform),
        ArgsSubcommand::Tmux => subcommand_tmux(platform),
    } {
        eprintln!("Error: {error}");
        if let Some(source) = error.source() {
            eprintln!("Source: {source}");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
