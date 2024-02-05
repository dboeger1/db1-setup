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

    configure_neovim(paths)?;

    println!("Done.");

    Ok(())
}
