use crate::{
    error::Error,
    platform::Platform,
};
use fs_extra::dir::{
    copy,
    CopyOptions,
};
use std::fs::{
    create_dir_all,
    metadata,
    remove_dir_all,
    remove_file,
};


#[derive(clap::Args, PartialEq, Eq)]
pub struct Args {
    #[arg(short, long)]
    pub force: bool,
}


pub(crate) fn subcommand_neovim(
    platform: &Platform,
    args: &Args,
) -> Result<(), Error> {
    let paths = &platform.neovim_paths;

    println!("Copying Neovim configuration...");
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
                "Missing directory: {}",
                paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    if !paths.source.is_dir() {
        return Err(Error {
            message: format!(
                "Not a directory: {}",
                paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Validate destination.
    if paths.destination.is_symlink() {
        match args.force {
            true => if let Err(error) = remove_file(&paths.destination) {
                return Err(Error {
                    message: format!(
                        "Failed to overwrite symlink: {}",
                        paths.destination.to_string_lossy(),
                    ),
                    source: Some(Box::new(error)),
                });
            },
            false => return Err(Error {
                message: format!(
                    "Cannot overwrite symlink: {}",
                    paths.destination.to_string_lossy(),
                ),
                source: None,
            }),
        }
    }

    if paths.destination.is_file() {
        match args.force {
            true => if let Err(error) = remove_file(&paths.destination) {
                return Err(Error {
                    message: format!(
                        "Failed to overwrite file: {}",
                        paths.destination.to_string_lossy(),
                    ),
                    source: Some(Box::new(error)),
                });
            },
            false => return Err(Error {
                message: format!(
                    "Cannot overwrite file: {}",
                    paths.destination.to_string_lossy(),
                ),
                source: None,
            }),
        }
    }

    if paths.destination.is_dir() {
        match args.force {
            true => if let Err(error) = remove_dir_all(&paths.destination) {
                return Err(Error {
                    message: format!(
                        "Failed to overwrite directory: {}",
                        paths.destination.to_string_lossy(),
                    ),
                    source: Some(Box::new(error)),
                });
            },
            false => return Err(Error {
                message: format!(
                    "Cannot overwrite directory: {}",
                    paths.destination.to_string_lossy(),
                ),
                source: None,
            }),
        }
    }

    // We should have either returned an error or deleted the existing file by
    // now. Maybe this should panic?
    if metadata(&paths.destination).is_ok() {
        return Err(Error {
            message: format!(
                "Cannot overwrite destination: {}",
                paths.destination.to_string_lossy(),
            ),
            source: None,
        });
    }

    // Create destination.
    create_dir_all(paths.destination.as_path())
        .map_err(|error| Error {
            message: format!(
                "Failed to create directory: {}",
                paths.destination.to_string_lossy(),
            ),
            source: Some(Box::new(error)),
        })?;

    // Copy contents from source to destination.
    copy(
        paths.source.as_path(),
        paths.destination.as_path(),
        &CopyOptions {
            content_only: true,
            ..Default::default()
        },
    )
        .map_or_else(
            |error| Err(Error {
                message: format!(
                    "Failed to copy directory contents: {} -> {}",
                    paths.source.to_string_lossy(),
                    paths.destination.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            }),
            |_| Ok(())
        )
}
