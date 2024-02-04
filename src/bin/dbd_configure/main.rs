mod args;
mod error;
mod platform;
mod source_destination;
mod subcommand;


use args::Args;
use clap::Parser;
use platform::{
    Platform,
    PLATFORM,
};
use std::{
    error::Error,
    process::ExitCode,
};
use subcommand::execute_subcommand;


fn main() -> ExitCode {
    // Get platform data.
    let platform: &Platform;
    match PLATFORM.as_ref() {
        Some(platform_data) => platform = platform_data,
        None => {
            eprintln!("Unrecognized platform. Aborting.");
            return ExitCode::FAILURE;
        },
    }

    // Parse arguments.
    let args = Args::parse();

    // Execute subcommand.
    if let Err(error) = execute_subcommand(platform, args.subcommand) {
        eprintln!("Error: {error}");
        if let Some(source) = error.source() {
            eprintln!("Source: {source}");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
