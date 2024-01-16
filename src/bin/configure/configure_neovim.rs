use crate::configure_error::ConfigureError;
use std::path::PathBuf;


pub(crate) fn configure_neovim() -> Result<(), ConfigureError> {
    let file = PathBuf::from("~/.config/nvim");
    if file.exists() {
        // error; neovim conf already exists
    }
    // copy
    Ok(())
}
