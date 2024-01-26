use crate::{
    error::Error,
    platform::Platform,
};
use std::fs::copy;


pub(crate) fn configure_tmux(platform_data: &Platform) -> Result<(), Error> {
    if platform_data.tmux_paths.is_none() {
        return Ok(());
    }
    let tmux_paths = platform_data
        .tmux_paths
        .as_ref()
        .unwrap();

    if !tmux_paths.source.exists() {
        return Err(Error {
            message: format!(
                "missing file: \"{}\"",
                tmux_paths.source.to_string_lossy(),
            ),
            source: None,
        });
    }

    if tmux_paths.destination.exists() {
        return Err(Error {
            message: format!(
                "cannot overwrite file: \"{}\"",
                tmux_paths.destination.to_string_lossy(),
            ),
            source: None,
        });
    }

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
