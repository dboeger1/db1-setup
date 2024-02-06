use crate::error::Error;
use print_command::run_and_print;
use std::process::Command;

pub(crate) fn configure_ssh() -> Result<(), Error> {
    let mut ssh_keygen_command = Command::new("ssh-keygen");
    ssh_keygen_command.args([
        "-t",
        "ed25519",
        "-C",
        "David Michael Boeger <dmboeger@gmail.com> [hostname]",
    ]);

    run_and_print(&mut ssh_keygen_command, false)
        .map_err(|error| Error {
            message: "Error running ssh-keygen command.".to_string(),
            source: Some(Box::new(error)),
        })?;

    println!("stuff");
    Ok(())
}
