use crate::configure_error::ConfigureError;
use dboeger1_dotfiles::INSTALL_TMUX_DIR;
use std::{
    fs::copy,
    path::PathBuf,
};


pub(crate) fn configure_tmux() -> Result<(), ConfigureError> {
    println!("Copying tmux configuration...");

    let file = PathBuf::from("~/.tmux.conf");
    if file.exists() {
        return Err(ConfigureError {
            message: format!(
                "destination already exists: \"{}\"",
                file.as_path().to_string_lossy(),
            ),
            source: None,
        });
    }

    if let Err(error) = copy(
        INSTALL_TMUX_DIR.join(".tmux.conf"),
        PathBuf::from("~/.tmux.conf"),
    ) {
        return Err(ConfigureError {
            message: format!(
                "failed to copy tmux configuration to destination: \"{}\"",
                "~/.tmux.conf",
            ),
            source: Some(error),
        });
    }

    println!("Done.");
    Ok(())
}
