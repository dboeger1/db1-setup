mod error;
//mod neovim;
mod platform;
mod source_destination;
//mod tmux;


//use neovim::configure_neovim;
//use tmux::configure_tmux;
use std::process::ExitCode;


#[macro_use]
extern crate lazy_static;


fn main() -> ExitCode {
    println!("{:#?}", *platform::PLATFORM);
    //if let Err(error) = install_packages() {
    //    eprintln!("{}", error);
    //    if let Some(source) = error.source {
    //        eprintln!("{}", source);
    //    }
    //    return ExitCode::FAILURE;
    //}

    //if let Err(error) = configure_tmux() {
    //    eprintln!("{}", error);
    //    if let Some(source) = error.source {
    //        eprintln!("{}", source);
    //    }
    //    return ExitCode::FAILURE;
    //}

    //if let Err(error) = configure_neovim() {
    //    eprintln!("{}", error);
    //    if let Some(source) = error.source {
    //        eprintln!("{}", source);
    //    }
    //    return ExitCode::FAILURE;
    //}

    ExitCode::SUCCESS
}
