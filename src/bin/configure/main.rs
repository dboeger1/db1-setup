mod configure_error;
mod configure_neovim;
mod configure_tmux;
mod install_dependencies;


use configure_neovim::configure_neovim;
use configure_tmux::configure_tmux;
use install_dependencies::install_dependencies;
use std::{process::ExitCode, error::Error};


fn main() -> ExitCode {
    if let Err(error) = install_dependencies() {
        eprintln!("{}", error);
        if let Some(source) = error.source() {
            eprintln!("{}", source);
        }
        return ExitCode::FAILURE;
    }

    if let Err(error) = configure_tmux() {
        eprintln!("{}", error);
        if let Some(source) = error.source() {
            eprintln!("{}", source);
        }
        return ExitCode::FAILURE;
    }

    if let Err(error) = configure_neovim() {
        eprintln!("{}", error);
        if let Some(source) = error.source() {
            eprintln!("{}", source);
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
