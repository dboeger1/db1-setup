use crate::{
    error::Error,
    platform::Platform,
};
use std::fs::copy;


pub(crate) fn configure_tmux(platform_data: &Platform) -> Result<(), Error> {
    // Check if platform supports tmux configuration.
    if platform_data.tmux_paths.is_none() {
        return Ok(());
    }
    let tmux_paths = platform_data
        .tmux_paths
        .as_ref()
        .unwrap();

    // Validate source.
    if !tmux_paths.source.exists() {
        return Err(Error {
            message: format!(
                "missing file: \"{}\"",
                tmux_paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    if !tmux_paths.source.is_file() {
        return Err(Error {
            message: format!(
                "not a regular file: \"{}\"",
                tmux_paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Validate destination.
    if tmux_paths.destination.exists() {
        return Err(Error {
            message: if tmux_paths.destination.is_file() {
                    format!(
                        "cannot overwrite file: \"{}\"",
                        tmux_paths.destination.to_string_lossy(),
                    )
                } else if tmux_paths.destination.is_dir() {
                    format!(
                        "cannot overwrite directory: \"{}\"",
                        tmux_paths.destination.to_string_lossy(),
                    )
                } else {
                    format!(
                        "cannot overwrite destination: \"{}\"",
                        tmux_paths.destination.to_string_lossy(),
                    )
                },
            source: None,
        });
    }

    // Copy source to destination.
    return copy(
        tmux_paths.source.as_path(),
        tmux_paths.destination.as_path(),
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "failed to copy file: \"{}\" -> \"{}\"",
                    tmux_paths.source.to_string_lossy(),
                    tmux_paths.destination.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(())
        );
}
