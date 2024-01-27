use crate::error::Error;
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

    println!(concat!(
        "Must run \"source ${{HOME}}/.cargo/env\" or restart shell for ",
        "environment changes to take effect.",
    ));
    Ok(())
}
