use crate::Error;
use std::{
    fs::{
        copy,
        metadata,
        remove_dir_all,
        remove_file,
    },
    path::{
        Path,
        PathBuf,
    },
};


#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[arg(short, long)]
    pub destination: Option<PathBuf>,

    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long)]
    pub source: Option<PathBuf>,
}


pub(super) fn configure(args: &Args) -> Result<(), Error> {
    // Determine source path.
    let path_source: &Path;
    if let Some(path_source_arg) = args.source.as_ref() {
        path_source = path_source_arg.as_path();
    // TODO: Consider default source path for unrecognized platforms.
    } else {
        return Err(Error {
            message: "--source required on platform with no default"
                .to_string(),
            source: None,
        });
    }

    // Determine destination path.
    let path_destination: &Path;
    if let Some(path_destination_arg) = args.destination.as_ref() {
        path_destination = path_destination_arg.as_path();
    // TODO: Consider default destination path for unrecognized platforms.
    } else {
        return Err(Error {
            message: "--destination required on platform with no default"
                .to_string(),
            source: None,
        });
    }

    // Check if source and destination are the same.
    if path_source == path_destination {
        return Err(Error {
            message: format!(
                "Source and destination are the same: {}",
                path_source.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Indicate operation.
    println!("Copying tmux configuration...");
    println!(
        "\tSource: {}",
        path_source.to_string_lossy(),
    );
    println!(
        "\tDestination: {}",
        path_destination.to_string_lossy(),
    );

    // Validate source.
    if !path_source.exists() {
        return Err(Error {
            message: format!(
                "Missing file: {}",
                path_source.to_string_lossy(),
            ),
            source: None,
        });
    }

    if !path_source.is_file() {
        return Err(Error {
            message: format!(
                "Not a regular file: {}",
                path_source.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Validate destination.
    if path_destination.is_symlink() {
        match args.force {
            true => if let Err(error) = remove_file(&path_destination) {
                return Err(Error {
                    message: format!(
                        "Failed to overwrite symlink: {}",
                        path_destination.to_string_lossy(),
                    ),
                    source: Some(Box::new(error)),
                });
            },
            false => return Err(Error {
                message: format!(
                    "Cannot overwrite symlink: {}",
                    path_destination.to_string_lossy(),
                ),
                source: None,
            }),
        }
    }

    if path_destination.is_file() {
        match args.force {
            true => if let Err(error) = remove_file(&path_destination) {
                return Err(Error {
                    message: format!(
                        "Failed to overwrite file: {}",
                        path_destination.to_string_lossy(),
                    ),
                    source: Some(Box::new(error)),
                });
            },
            false => return Err(Error {
                message: format!(
                    "Cannot overwrite file: {}",
                    path_destination.to_string_lossy(),
                ),
                source: None,
            }),
        }
    }

    if path_destination.is_dir() {
        match args.force {
            true => if let Err(error) = remove_dir_all(&path_destination) {
                return Err(Error {
                    message: format!(
                        "Failed to overwrite directory: {}",
                        path_destination.to_string_lossy(),
                    ),
                    source: Some(Box::new(error)),
                });
            },
            false => return Err(Error {
                message: format!(
                    "Cannot overwrite directory: {}",
                    path_destination.to_string_lossy(),
                ),
                source: None,
            }),
        }
    }

    // We should have either returned an error or deleted the existing file by
    // now. Maybe this should panic?
    if metadata(&path_destination).is_ok() {
        return Err(Error {
            message: format!(
                "Cannot overwrite destination: {}",
                path_destination.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Copy source to destination.
    copy(path_source, path_destination)
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "Failed to copy file: {} -> {}",
                    path_source.to_string_lossy(),
                    path_destination.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(())
        )
}
