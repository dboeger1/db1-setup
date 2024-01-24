use crate::configure_error::Error;
use dboeger1_dotfiles::*;
use std::fs::copy;


pub(crate) fn configure_tmux() -> Result<(), Error> {
    println!("Copying tmux configuration...");

    let source = INSTALL_TMUX_DIR.join(".tmux.conf");
    let destination = HOME_DIR.join(".tmux.conf");

    if destination.exists() {
        return Err(Error {
            message: format!(
                "destination already exists: \"{}\"",
                destination.to_string_lossy(),
            ),
            source: None,
        });
    }

    if let Err(error) = copy(source, &destination) {
        return Err(Error {
            message: format!(
                "failed to copy tmux configuration to destination: \"{}\"",
                destination.to_string_lossy(),
            ),
            source: Some(error),
        });
    }

    println!("Done.");
    Ok(())
}
