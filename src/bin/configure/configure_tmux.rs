use crate::configure_error::ConfigureError;
use std::path::PathBuf;


pub(crate) fn configure_tmux() -> Result<(), ConfigureError> {
    let file = PathBuf::from("~/.tmux.conf");
    if file.exists() {
        // error; tmux conf already exists
    }
    // copy
    Ok(())
}
