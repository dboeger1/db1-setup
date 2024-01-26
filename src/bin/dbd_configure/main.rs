mod error;
mod neovim;
mod platform;
mod source_destination;
mod tmux;


use crate::{
    neovim::configure_neovim,
    tmux::configure_tmux,
};
use platform::PLATFORM;
use std::{
    error::Error,
    process::ExitCode,
};


fn main() -> ExitCode {
    // Get platform data.
    if PLATFORM.is_none() {
        eprintln!("unrecognized platform");

        return ExitCode::FAILURE;
    }
    let platform_data = PLATFORM.unwrap();

    // Install helpful packages.
    if let Some(install_packages) = platform_data.install_packages {
        if let Err(error) = install_packages() {
            eprintln!("{}", error);
            if let Some(source) = error.source() {
                eprintln!("{}", source);
            }
            return ExitCode::FAILURE;
        }
    }

    // Configure tmux.
    if let Err(error) = configure_tmux(&platform_data) {
        eprintln!("{}", error);
        if let Some(source) = error.source() {
            eprintln!("{}", source);
        }
        return ExitCode::FAILURE;
    }

    // Configure neovim.
    if let Err(error) = configure_neovim(&platform_data) {
        eprintln!("{}", error);
        if let Some(source) = error.source() {
            eprintln!("{}", source);
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
