use std::fs::create_dir_all;

use crate::{
    error::Error,
    platform::Platform,
};
use fs_extra::dir::{
    copy,
    CopyOptions,
};


pub(crate) fn configure_neovim(platform_data: &Platform) -> Result<(), Error> {
    // Check if platform supports neovim configuration.
    if platform_data.neovim_paths.is_none() {
        return Ok(());
    }
    let neovim_paths = platform_data
        .neovim_paths
        .as_ref()
        .unwrap();

    // Validate source.
    if !neovim_paths.source.exists() {
        return Err(Error {
            message: format!(
                "missing directory: \"{}\"",
                neovim_paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    if !neovim_paths.source.is_dir() {
        return Err(Error {
            message: format!(
                "not a directory: \"{}\"",
                neovim_paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Validate destination.
    if neovim_paths.destination.exists() {
        return Err(Error {
            message: if neovim_paths.destination.is_file() {
                    format!(
                        "cannot overwrite file: \"{}\"",
                        neovim_paths.destination.to_string_lossy(),
                    )
                } else if neovim_paths.destination.is_dir() {
                    format!(
                        "cannot overwrite directory: \"{}\"",
                        neovim_paths.destination.to_string_lossy(),
                    )
                } else {
                    format!(
                        "cannot overwrite destination: \"{}\"",
                        neovim_paths.destination.to_string_lossy(),
                    )
                },
            source: None,
        });
    }

    // Create destination.
    create_dir_all(neovim_paths.destination.as_path())
        .map_err(|error| Error {
            message: format!(
                "failed to create directory: \"{}\"",
                neovim_paths.destination.to_string_lossy(),
            ),
            source: Some(Box::new(error)),
        })?;

    // Copy contents from source to destination.
    return copy(
        neovim_paths.source.as_path(),
        neovim_paths.destination.as_path(),
        &CopyOptions {
            content_only: true,
            ..Default::default()
        },
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "failed to copy directory contents: \"{}\" -> \"{}\"",
                    neovim_paths.source.to_string_lossy(),
                    neovim_paths.destination.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(())
        );
}
