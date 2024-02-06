mod error;
mod platform;
mod source_destination;
mod subcommand;


use clap::Parser;
use error::Error;
use platform::{
    Platform,
    PLATFORM,
};
use subcommand::{
    execute_subcommand,
    Subcommand,
};


#[derive(Parser)]
#[command(name = env!("CARGO_CRATE_NAME"))]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}


pub fn execute(args: &Args) -> Result<(), Error> {
    // Get platform data.
    let platform: &Platform;
    match PLATFORM.as_ref() {
        Some(platform_data) => platform = platform_data,
        None => return Err(Error {
            message: "Unrecognized platform. Aborting.".to_string(),
            source: None,
        }),
    }

    // Execute subcommand.
    execute_subcommand(platform, &args.subcommand)
}
