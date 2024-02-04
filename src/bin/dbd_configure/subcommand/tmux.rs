mod args;
mod configure;


use configure::configure_tmux;
use crate::{
    error::Error,
    platform::Platform,
};

pub(crate) use args::Args;


pub(crate) fn subcommand_tmux(
    platform: &Platform,
    _args: &Args,
) -> Result<(), Error> {
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
