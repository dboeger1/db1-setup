mod args;
mod configure;


use configure::configure_neovim;
use crate::{
    error::Error,
    platform::Platform,
};

pub(crate) use args::Args;


pub(crate) fn subcommand_neovim(
    platform: &Platform,
    _args: &Args,
) -> Result<(), Error> {
    if let Some(neovim_paths) = platform.neovim_paths.as_ref() {
        println!("Copying Neovim configuration...");
        println!(
            "\tSource: {}",
            neovim_paths.source.to_string_lossy(),
        );
        println!(
            "\tDestination: {}",
            neovim_paths.destination.to_string_lossy(),
        );

        configure_neovim(neovim_paths)?;

        println!("Done.");
    }

    Ok(())
}
