mod args;


use crate::{
    error::Error,
    platform::Platform,
};
use std::fs::copy;

pub(crate) use args::Args;


pub(crate) fn subcommand_tmux(
    platform: &Platform,
    _args: &Args,
) -> Result<(), Error> {
    let paths = &platform.tmux_paths;

    println!("Copying tmux configuration...");
    println!(
        "\tSource: {}",
        paths.source.to_string_lossy(),
    );
    println!(
        "\tDestination: {}",
        paths.destination.to_string_lossy(),
    );

    // Validate source.
    if !paths.source.exists() {
        return Err(Error {
            message: format!(
                "Missing file: {}",
                paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    if !paths.source.is_file() {
        return Err(Error {
            message: format!(
                "Not a regular file: {}",
                paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Validate destination.
    if paths.destination.exists() {
        return Err(Error {
            message: if paths.destination.is_file() {
                    format!(
                        "Cannot overwrite file: {}",
                        paths.destination.to_string_lossy(),
                    )
                } else if paths.destination.is_dir() {
                    format!(
                        "Cannot overwrite directory: {}",
                        paths.destination.to_string_lossy(),
                    )
                } else {
                    format!(
                        "Cannot overwrite destination: {}",
                        paths.destination.to_string_lossy(),
                    )
                },
            source: None,
        });
    }

    // Copy source to destination.
    copy(
        paths.source.as_path(),
        paths.destination.as_path(),
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "Failed to copy file: {} -> {}",
                    paths.source.to_string_lossy(),
                    paths.destination.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(())
        )
}
