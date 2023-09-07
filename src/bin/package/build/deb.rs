use crate::PackageError;
use dboeger1_dotfiles::*;
use std::fs::{
    copy,
    create_dir_all,
};


pub(crate) fn build_deb() -> Result<(), PackageError> {
    create_dir_all(PACKAGES_DPKG_DIR.as_path())
        .map_err(|error| PackageError {
            message: format!(
                "failed to create deb directory \"{}\"",
                PACKAGES_DPKG_DIR.to_string_lossy(),
            ),
            source: Some(error),
        })?;

    for (source, destination) in [
        (
            PACKAGES_SRC_FILE.as_path(),
            PACKAGES_DPKG_SRC_FILE.as_path(),
        ),
        (
            ASSETS_DEB_CONTROL_FILE.as_path(),
            PACKAGES_DPKG_CONTROL_FILE.as_path(),
        ),
    ] {
        copy(source, destination)
            .map_or_else(
                |error| Err(PackageError {
                    message: format!(
                        "failed to copy file \"{}\" to \"{}\"",
                        source.to_string_lossy(),
                        destination.to_string_lossy(),
                    ),
                    source: Some(error),
                }),
                |_| Ok(()),
            )?;
    }

    Ok(())
}
