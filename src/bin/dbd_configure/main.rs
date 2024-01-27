mod error;
mod neovim;
mod platform;
mod source_destination;
mod tmux;


use crate::{
    neovim::configure_neovim,
    tmux::configure_tmux,
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
    let mut return_value = ExitCode::SUCCESS;

    // Get platform data.
    println!();
    println!("Detecting platform...");

    let platform: &Platform;
    match PLATFORM.as_ref() {
        Some(platform_data) => platform = platform_data,
        None => {
            eprintln!("Unrecognized platform. Aborting.");
            eprintln!();
            return ExitCode::FAILURE;
        },
    }

    println!("Done.");

    // Install helpful packages.
    if let Some(install_packages) = platform.install_packages {
        println!();
        println!("Installing helpful packages...");

        if let Err(error) = install_packages() {
            eprintln!("Error: {error}");
            if let Some(source) = error.source() {
                eprintln!("Source: {source}");
            }
            return_value = ExitCode::FAILURE;
        } else {
            println!("Done.");
        }
    }

    // Configure tmux.
    if let Some(tmux_paths) = platform.tmux_paths.as_ref() {
        println!();
        println!("Copying tmux configuration...");

        if let Err(error) = configure_tmux(tmux_paths) {
            eprintln!("Error: {error}");
            if let Some(source) = error.source() {
                eprintln!("Source: {source}");
            }
            return_value = ExitCode::FAILURE;
        } else {
            println!("Done.");
        }
    }

    // Configure neovim.
    if let Some(neovim_paths) = platform.neovim_paths.as_ref() {
        println!();
        println!("Copying neovim configuration...");

        if let Err(error) = configure_neovim(neovim_paths) {
            eprintln!("Error: {error}");
            if let Some(source) = error.source() {
                eprintln!("Source: {source}");
            }
            return_value = ExitCode::FAILURE;
        } else {
            println!("Done.");
        }
    }

    return_value
}
