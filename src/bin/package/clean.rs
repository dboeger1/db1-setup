use crate::{
    error::Error,
    values::DIR_PROJECT_PACKAGES,
};
use std::fs::remove_dir_all;


pub(crate) fn clean() -> Result<(), Error> {
    if let Err(error) = remove_dir_all(DIR_PROJECT_PACKAGES.as_path()) {
        return Err(Error {
            message: format!(
                "failed to remove directory: {}",
                DIR_PROJECT_PACKAGES.to_string_lossy(),
            ),
            source: Some(error),
        });
    }

    Ok(())
}
