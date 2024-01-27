use crate::error::Error;
use dboeger1_dotfiles::HOME_DIR;
use print_command::run_and_print;
use std::process::Command;


pub(crate) fn rustup_init() -> Result<(), Error> {
    let mut rustup_init_command = Command::new("rustup-init");
    rustup_init_command.arg("-y");

    run_and_print(&mut rustup_init_command, false)
        .map_err(|error| Error {
            message: "Error running rustup-init command.".to_string(),
            source: Some(Box::new(error)),
        })?;

    let mut source_command = Command::new("source");
    source_command.arg(HOME_DIR.join(".cargo/env"));

    run_and_print(&mut source_command, false)
        .map_err(|error| Error {
            message: "Error running source command.".to_string(),
            source: Some(Box::new(error)),
        })
}
