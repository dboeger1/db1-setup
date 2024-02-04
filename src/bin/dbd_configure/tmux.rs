use crate::{
    error::Error,
    platform::Platform,
    source_destination::SourceDestination,
};
use std::fs::copy;


pub(crate) fn subcommand_tmux(platform: &Platform) -> Result<(), Error> {
    if let Some(tmux_paths) = platform.tmux_paths.as_ref() {
        println!("Copying tmux configuration...");
        println!(
            "\tSource: {}",
            tmux_paths.source.to_string_lossy(),
        );
        println!(
            "\tDestination: {}",
            tmux_paths.destination.to_string_lossy(),
        );

        configure_tmux(tmux_paths)?;

        println!("Done.");
    }

    Ok(())
}

fn configure_tmux(
    tmux_paths: &SourceDestination,
) -> Result<(), Error> {
    // Validate source.
    if !tmux_paths.source.exists() {
        return Err(Error {
            message: format!(
                "Missing file: {}",
                tmux_paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    if !tmux_paths.source.is_file() {
        return Err(Error {
            message: format!(
                "Not a regular file: {}",
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
                        "Cannot overwrite file: {}",
                        tmux_paths.destination.to_string_lossy(),
                    )
                } else if tmux_paths.destination.is_dir() {
                    format!(
                        "Cannot overwrite directory: {}",
                        tmux_paths.destination.to_string_lossy(),
                    )
                } else {
                    format!(
                        "Cannot overwrite destination: {}",
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
                    "Failed to copy file: {} -> {}",
                    tmux_paths.source.to_string_lossy(),
                    tmux_paths.destination.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(())
        );
}
