use crate::configure_error::ConfigureError;
use dboeger1_dotfiles::INSTALL_NEOVIM_DIR;
use std::{
    fs::copy,
    path::PathBuf,
};


pub(crate) fn configure_neovim() -> Result<(), ConfigureError> {
    println!("Copying neovim configuration...");

    let file = PathBuf::from("~/.config/nvim");
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
        INSTALL_NEOVIM_DIR.as_path(),
        PathBuf::from("~/.config/nvim"),
    ) {
        return Err(ConfigureError {
            message: format!(
                "failed to copy neovim configuration to destination: \"{}\"",
                "~/.config/nvim",
            ),
            source: Some(error),
        });
    }

    println!("Done.");
    Ok(())
}
