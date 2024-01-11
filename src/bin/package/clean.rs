use crate::package_error::PackageError;
use dboeger1_dotfiles::*;
use std::fs::remove_dir_all;


pub(crate) fn clean() -> Result<(), PackageError> {
    match remove_dir_all(PROJECT_PACKAGES_DIR.as_path()) {
        Ok(_) => Ok(()),
        Err(error) => Err(PackageError {
            message: format!(
                "failed to remove packages directory \"{}\"",
                PROJECT_PACKAGES_DIR.to_string_lossy(),
            ),
            source: Some(error),
        }),
    }
}
