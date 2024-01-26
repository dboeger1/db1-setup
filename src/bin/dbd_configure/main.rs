mod error;
mod platform;
mod source_destination;
mod tmux;


use platform::PLATFORM;
use std::{
    error::Error,
    process::ExitCode,
};

use crate::tmux::configure_tmux;


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

    // Copy neovim configuration.
    println!("neovim paths:");
    println!(
        "\t{}",
        platform_data
            .neovim_paths
            .as_ref()
            .unwrap()
            .source
            .to_string_lossy(),
    );
    println!(
        "\t{}",
        platform_data
            .neovim_paths
            .as_ref()
            .unwrap()
            .destination
            .to_string_lossy(),
    );

    ExitCode::SUCCESS
}
