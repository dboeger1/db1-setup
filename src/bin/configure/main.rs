mod configure_error;
//mod configure_neovim;
//mod configure_tmux;
mod install_dependencies;


use install_dependencies::install_dependencies;
use std::process::ExitCode;


fn main() -> ExitCode {
    if install_dependencies().is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
